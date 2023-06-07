use std::io::Write;
use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    //localhostの0番ポートを指定するとOSが空いてるポートを割り振ってくれるので便利な書き方
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    loop {
        let mut input = String::new();

        print!("入力: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;

        //ここで引数から渡されたアドレス宛てにデータを送っている
        socket.send_to(input.as_bytes(), address)?;

        //echo部分
        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!("Echo: {}", str::from_utf8(&buffer).expect("faild tot convert to String"));
    }
}