fn main() {
    // list all of the devices pcap tells us are available
    for device in pcap::Device::list().unwrap() {
        println!("Found device! {:?}", device);

        // now you can create a Capture with this Device if you want.
        let mut cap = pcap::Capture::from_device(device)
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        // get a packet from this capture
        let packet = cap.next();

        println!("got a packet! {:?}", packet);
    }
}
