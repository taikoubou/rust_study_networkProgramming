use std::io::{
    self, BufRead, BufReader, Write
};
use std::net::TcpStream;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error> {
    //指定のアドレスに対して接続を試みている
    //この時点で3 way handshakeが行われている
    let mut stream = TcpStream::connect(address)?;

    loop {
        let mut input = String::new();

        print!("入力: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        //ネットワークではバイトストリームを送受信するため入出力する際はUTF8に変換する必要がある
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("Echo: {}\n", str::from_utf8(&buffer)?);
    }
}