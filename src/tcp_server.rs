use std::io::{
    Read, Write
};
use std::net::{
    TcpListener, TcpStream
};
use std::{
    str, thread
};

//引数からアドレスを受け取り接続を待つ関数
pub fn serve(address: &str) -> Result<(), failure::Error> {
    //このlistenerはクライアントからのコネクション確立要求を待ち受ける役割
    //クライアントからコネクション確立要求がきたら3 way handshakeを行い、確立済みのソケットがカーネル内部のキューに保存される
    let listener = TcpListener::bind(address)?;

    loop {
        //accept()は確立済みのソケットを返却し、ソケットが存在しなかったらスレッドを停止する関数
        //つまり、streamにはコネクション確立済みのソケットが格納される
        //listenerはリスニングソケット、streamは接続済みソケットと呼ぶことにする
        //文献によってはlistenerをサーバーソケット、streamをクライアントソケットと呼ぶこともある
        let (stream, _) = listener.accept()?;

        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}


//クライアントからの入力を待ち、受信したものを返却する関数
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);

    let mut buffer = [0u8; 1024];

    loop {
        //read()はstreamにデータが流れてくるまでスレッドを停止し、データがきたらbufferにデータを格納しデータサイズを返却する関数
        //read()はEOFに達すると0を返却し、通信の終了を表す
        //このときstreamも解放されるためコネクションが解放される
        let nbytes = stream.read(&mut buffer)?;

        if nbytes == 0 {
            debug!("Connection closed.");

            return Ok(());
        }

        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes]);
    }
}