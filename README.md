This is a fantastic project. Building a "toy" version is the best way to learn. Let's make a plan.

The key is to _not_ build everything at once. We'll separate the project into two main parts: the **Data Plane** (getting a `ping` to work) and the **Control Plane** (managing the user and session).

We'll start with the Data Plane, and we'll "hard-code" all the connections first, just to make data flow.

Here is a 4-milestone plan.

### Milestone 1: The "Core" Data Tunnel (S-GW + P-GW)

Your first goal is to build the "GTP-U Tunnel." This is two programs that shuffle data between a virtual "internet" and a virtual "radio" network.

-   **Component 1: The `pgw` (P-GW)**

    -   **Job:** This program is your "exit ramp" to the internet.
    -   **Task:**
        1.  Create a **TUN/TAP device** (e.g., `tun0`). This is your "internet."
        2.  Listen on a UDP port (e.g., 2152) for "GTP-U" packets.
        3.  **Forwarding (Uplink):** When a packet arrives, _de-capsulate_ it (strip the GTP-U header) and write the inner IP packet to `tun0`.
        4.  **Forwarding (Downlink):** When an IP packet arrives _from_ `tun0` (like a ping reply), _en-capsulate_ it (wrap it in a GTP-U header) and send it over UDP to the S-GW's address.

-   **Component 2: The `sgw` (S-GW)**
    -   **Job:** For V1, this is a simple "data router."
    -   **Task:**
        1.  Listen on one UDP port for the eNodeB (e.g., 2153).
        2.  Listen on another for the P-GW (e.g., 2154).
        3.  **Forwarding (Uplink):** When a packet arrives from the eNodeB, forward it to the P-GW.
        4.  **Forwarding (Downlink):** When a packet arrives from the P-GW, forward it to the eNodeB.

**✅ End of Milestone 1:** You can send a GTP-U packet to your `sgw` and see it come out the `pgw`'s `tun0` device.

---

### Milestone 2: The "Radio" Link (UE + eNodeB)

Now we build the two ends of the "radio" link. We'll use a simple UDP socket as our "simulated air."

-   **Component 3: The `ue` (Phone)**

    -   **Job:** This is the "client" machine.
    -   **Task:**
        1.  Create its _own_ `tun0` device. This is the "phone's" IP stack.
        2.  **Forwarding (Uplink):** When an IP packet (like a `ping`) arrives _from_ its `tun0`, send this _raw IP packet_ over a UDP socket (our "air") to the eNodeB.
        3.  **Forwarding (Downlink):** When a packet arrives _from_ the "air," write it to `tun0`.

-   **Component 4: The `enb` (eNodeB)**
    -   **Job:** This is your "tower." It's the bridge between the "air" and the "core."
    -   **Task:**
        1.  Listen on a UDP port for the "air" (from the `ue`).
        2.  **Forwarding (Uplink):** When a raw IP packet arrives from the `ue`, _en-capsulate_ it into a GTP-U packet and send it to the `sgw`'s UDP port.
        3.  **Forwarding (Downlink):** When a GTP-U packet arrives from the `sgw`, _de-capsulate_ it and send the inner IP packet to the `ue` over the "air" (UDP).

**✅ End of Milestone 2:** You can `ping` from the `ue`'s `tun0` device, and the ping will go _all the way_ through `enb` -> `sgw` -> `pgw` -> `pgw`'s `tun0` and back!

---

### Milestone 3: The "Brains" (MME + HSS)

Right now, all the IP addresses are hard-coded. Now we add the "brains" to set up the session automatically. This is the "Control Plane."

-   **Component 5: The `hss` (Database)**

    -   **Job:** A simple user database.
    -   **Task:** Create a simple server (e.g., TCP) that listens for a "user ID" and responds with "OK, here are their security keys." (We can skip the complex `Diameter` protocol and just invent our own simple JSON-based API).

-   **Component 6: The `mme` (Orchestrator)**
    -   **Job:** This is the most complex part. It manages the "Attach Procedure."
    -   **Task:**
        1.  Listen for connections from the `enb` (this is the S1-MME interface).
        2.  When the `ue` connects, the `enb` will forward its "Attach Request" to the `mme`.
        3.  The `mme` then performs the "login" sequence:
            -   Asks the `hss` if the user is valid.
            -   Tells the `sgw` and `pgw` to "Create Session" (using the **GTP-C** protocol—a new protocol you'll need to implement!).
            -   Tells the `enb` to activate the "radio bearer" (the pipe) for the `ue`.

**✅ End of Milestone 3:** A user can "boot" the `ue` program, and it will automatically attach and get an IP address, _without_ hard-coding.

---

### Milestone 4: The "Real" Radio Protocols (RLC, MAC, PDCP)

Finally, we make our "simulated air" (the UDP socket) imperfect.

-   **Job:** Go back into the `ue` and `enb` programs.
-   **Task:**
    1.  Instead of just sending the IP packet over UDP, you first pass it to a **PDCP** "library" (which you write) to encrypt it.
    2.  Then, you pass it to an **RLC** "library" (which you write) to segment it (slice it into smaller pieces).
    3.  This is when you can use the **`tc` (Traffic Control)** tool we discussed. You'll run `tc` on your `enb`'s "air" link to _randomly drop 5% of packets_.
    4.  You can then test if your **RLC** layer's re-transmission logic kicks in and successfully re-sends the lost pieces.

**✅ End of Milestone 4:** You have a working, miniature LTE network that can handle packet loss, just like the real thing.

---

This is a big, exciting, and very rewarding project.

Does it make sense to start with **Milestone 1**, just focusing on building that `pgw` and `sgw` data tunnel?
