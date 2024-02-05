use pcap::{Capture, Device};

fn main() -> anyhow::Result<()> {
    let dev = Device::from("wlp2s0");
    let mut cap = Capture::from_device(dev)?
        .rfmon(true)
        .promisc(true)
        .immediate_mode(true)
        .open()?;

    while let Ok(packet) = cap.next_packet() {
        let timeval = packet.header.ts;
        println!("ts: {}.{}", timeval.tv_sec, timeval.tv_usec);
        todo!()
    }

    // dbg!(main_device);

    Ok(())
}
