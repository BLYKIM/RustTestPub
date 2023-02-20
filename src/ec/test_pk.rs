use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use data_encoding::BASE64;
use pdu::{Ethernet, EthernetPdu};

fn testpk() {
    let dec = base64_engine.decode("//".as_bytes()).unwrap();
    println!("{dec:?}");
    let aice = BASE64.decode("MjAyMi0xMi0yOFQwNzozOTozMi42OTE2MDlaIEVSUk9SIFRMUyBhbGVydCByZWNlaXZlZDogQWxlcnRNZXNzYWdlUGF5bG9hZCB7".as_bytes()).unwrap();
    println!("{aice:?}");
    let aiceto = std::str::from_utf8(&aice);
    println!("{aiceto:?}");

    let lalala: &[u8] = &[];

    println!("{}", lalala.len());
    let testpack: &[u8] = &[
        0x68, 0x5b, 0x35, 0xc0, 0x61, 0xb6, 0x00, 0x1d, 0x09, 0x94, 0x65, 0x38, 0x08, 0x00, 0x45,
        0x00, 0x00, 0x3b, 0x2d, 0xfd, 0x00, 0x00, 0x40, 0x11, 0xbc, 0x43, 0x83, 0xb3, 0xc4, 0x2e,
        0x83, 0xb3, 0xc4, 0xdc, 0x18, 0xdb, 0x18, 0xdb, 0x00, 0x27, 0xe0, 0x3e, 0x05, 0x1d, 0x07,
        0x15, 0x08, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x08, 0x07, 0x74, 0x65, 0x73,
        0x74, 0x41, 0x70, 0x70, 0x08, 0x01, 0x31, 0x0a, 0x04, 0x1e, 0xcc, 0xe2, 0x51,
    ];
    println!("{testpack:?}");

    println!("\n\n");
    let testpacket = EthernetPdu::new(testpack).unwrap();
    println!("{:?}", testpacket.inner());
    let packet = EthernetPdu::new(&aice).unwrap();
    println!("{:?}", packet.inner());
    match testpacket.inner() {
        Ok(Ethernet::Ipv4(_)) => println!("ipv4"),
        Ok(Ethernet::Ipv6(_)) => println!("ipv6"),
        Ok(other) => println!("unexpected protocol, {other:?}"),
        Err(e) => println!("parser failure: {e:?}"),
    }
    let src = packet.source_address();
    let dst = packet.destination_address();
    // let proto = format!("protocol: 0x{:02x}", packet.protocol());

    // println!("{src:?}, {dst:?}, {proto}");

    let asd = std::str::from_utf8(&dec[..10]);
    println!("{asd:?}");
}
