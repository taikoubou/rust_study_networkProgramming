fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        error!("Please specify target interface name.");
        std::process::exit(1);
    }

    let interface_name = &args[1];
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| iface.name == *interface_name).expect("Failed to get interface.");

    //txは転送機、rxは受信機
    //TRPLのマルチスレッドの部分で出てくる
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type."),
        Err(e) => panic!("Failed to create datalink channel {}.", e);
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                //frame(フレーム)はデータリンクのパケットを表す
                let frame = EthernetPacket::new(frame).unwrap();

                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not and IPv4 or IPv6 packet.");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}.", e);
            }
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHanderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHanderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP or UDP packet.");
            }
        }
    }
}

fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHanderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHanderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP or UDP packet.");
            }
        }
    }
}

fn tcp_handler(packet: &GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

fn udp_handler(packet: &GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());

    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}

fn print_packet_info(l3: &GettableEndPoints, l4: &GettableEndPoints, proto: &str) {
    println!("Captured a packet from {}|{} to {}|{}\n", proto, l3.get_source(), l4.get_source(), l3.get_destination(), l4.get_destination());

    let payload = l4.get_payload();
    let len = payload.len();

    for i in 0..len {
        print!("{:<02X}", payload[i]);

        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _j in 0..WIDTH - 1 - (i % WIDTH) {
                print!(" ");
            }
            print!("| ");

            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}