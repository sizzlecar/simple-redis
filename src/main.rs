use std::sync::Arc;

use anyhow::Result;
use futures::SinkExt;
use simple_redis::Processor;
use simple_redis::{network::RespFrameCodec, process::CommandGroup};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};

/// 1. 从TcpStream从读取frame，要为Resp实现 frame decode 和 encode
/// 2. 从frame中解析出命令和参数
/// 3. 根据命令和参数调用对应的Processor
/// 4. Processor返回一个Resp，将Resp encode 结果写入TcpStream
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let addr = "127.0.0.1:6379";
    let listener: TcpListener = TcpListener::bind(addr).await?;
    info!("🚀 Simple Redis Server listening on: {}", addr);
    let data_arc = Arc::new(simple_redis::Data::new());
    loop {
        let (socket, addr) = listener.accept().await?;
        info!("📡 New client connected: {}", addr);
        let data_clone = data_arc.clone();
        tokio::spawn(async move {
            let mut framed = Framed::new(socket, RespFrameCodec);
            // In a loop, read data from the socket and write the data back.
            loop {
                match framed.next().await {
                    Some(Ok(frame)) => {
                        info!("📥 Received frame from {}: {:?}", addr, frame);
                        let command: CommandGroup = CommandGroup::try_from(frame)?;
                        info!("⚡ Processing command from {}: {:?}", addr, command);
                        let res_frame = command.process(&data_clone)?;
                        info!("📤 Response to {}: {:?}", addr, res_frame);
                        match framed.send(res_frame).await {
                            Ok(_) => {
                                debug!("✅ Response sent successfully to {}", addr);
                            }
                            Err(e) => {
                                error!("❌ Failed to send response to {}: {}", addr, e);
                                return Err(e);
                            }
                        };
                    }
                    Some(Err(e)) => {
                        error!("🔥 Frame decode error from {}: {}", addr, e);
                        return Err(e);
                    }
                    None => {
                        info!("👋 Client {} disconnected", addr);
                        return Ok(());
                    }
                };
            }
        });
    }
}
