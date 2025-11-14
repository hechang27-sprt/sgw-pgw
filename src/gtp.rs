use std::{hash::Hash, sync::atomic::AtomicU32, time::Duration};

use futures::future::FutureExt;
use scc2::{HashMap, hash_map::Entry};
use sharded_slab::{Entry as SlabEntry, Slab};
use tokio::time::sleep;

// A 2-way associative container that associates a TEID with a key and some resource
// as well as the key with the resource and the corresponding TEID
// the key uniquely identifies a request from outside the EPC and is associated with the local TEID and some network destination/resources inside or outside the EPC
// the TEID unique identifies a request from within the EPC and is associcated with the request from outside the EPC as well as some network destination/resources inside or outside the EPC.

type Teid = u32;
pub struct GtpController<K: Eq + Hash + Clone, T> {
    forward: HashMap<Teid, (K, usize)>,
    backward: HashMap<K, (Teid, usize)>,
    teid_counter: AtomicU32,
    store: Slab<T>,
}

impl<K, T> GtpController<K, T>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            backward: HashMap::new(),
            teid_counter: fastrand::u32(..u32::MAX).into(),
            store: Slab::new(),
        }
    }

    async fn reset_counter(&self) {
        let mut teid;
        while {
            teid = fastrand::u32(..u32::MAX);
            self.forward.contains_async(&teid).await
        } {
            // noop
        }

        self.teid_counter
            .store(teid, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn add_resource(&self, key: K, res: T, duration: Duration) -> Option<Teid> {
        let teid = self
            .teid_counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        enum Cause {
            Timeout,
            DuplicateId,
            DuplicateKey,
        }

        let timer = sleep(duration).map(|_| Result::<(), Cause>::Err(Cause::Timeout));

        fn filter_vacant<K, V>(
            entry: Entry<'_, K, V>,
            err: Cause,
        ) -> Result<Entry<'_, K, V>, Cause> {
            if matches!(entry, Entry::Vacant(_)) {
                Result::Ok(entry)
            } else {
                Result::Err(err)
            }
        }

        let entry_f_fut = self
            .forward
            .entry_async(teid)
            .map(|e| filter_vacant(e, Cause::DuplicateId));
        let entry_b_fut = self
            .backward
            .entry_async(key.clone())
            .map(|e| filter_vacant(e, Cause::DuplicateKey));

        match tokio::try_join!(timer, entry_f_fut, entry_b_fut) {
            Ok((_, Entry::Vacant(f), Entry::Vacant(b))) => {
                let res = self.store.insert(res).unwrap();
                f.insert_entry((key.clone(), res));
                b.insert_entry((teid, res));
                Some(teid)
            }
            Err(Cause::DuplicateId) => {
                self.reset_counter().await;
                None
            }
            Err(Cause::DuplicateKey) => {
                panic!("Duplicate Resource Key Added!");
            }
            Err(Cause::Timeout) => None,
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn get_with_key(&self, key: K) -> Option<(Teid, SlabEntry<'_, T>)> {
        self.backward
            .read_async(&key, |_, &(teid, res_key)| {
                (teid, self.store.get(res_key).expect("Invalid Resource Key"))
            })
            .await
    }

    pub async fn get(&self, teid: Teid) -> Option<(K, SlabEntry<'_, T>)> {
        self.forward
            .read_async(&teid, |_, (key, res_key)| {
                (
                    key.clone(),
                    self.store.get(*res_key).expect("Invalid Resource Key"),
                )
            })
            .await
    }
}

impl<K, T> Default for GtpController<K, T>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
