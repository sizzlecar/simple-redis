use anyhow::Result;
use futures::SinkExt;
use simple_redis::{network::RespFrameCodec, Resp, SimpleStringsData};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

/// 1. 从TcpStream从读取frame，要为Resp实现 frame decode 和 encode
/// 2. 从frame中解析出命令和参数
/// 3. 根据命令和参数调用对应的Processor
/// 4. Processor返回一个Resp，将Resp encode 结果写入TcpStream
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            let mut framed = Framed::new(socket, RespFrameCodec);
            // In a loop, read data from the socket and write the data back.
            loop {
                match framed.next().await {
                    Some(Ok(frame)) => {
                        println!("Received frame: {:?}", frame);
                        //let resp = frame.process().await?;
                        let response =
                            Resp::SimpleStrings(SimpleStringsData::new("OK".to_string()));
                        match framed.send(response).await {
                            Ok(_) => {
                                println!("Response sent");
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                return Err(e);
                            }
                        };
                    }
                    Some(Err(e)) => {
                        eprintln!("Error: {}", e);
                        return Err(e);
                    }
                    None => {
                        println!("Connection closed");
                        return Ok(());
                    }
                };
            }
        });
    }
}
