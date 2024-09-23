
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};

#[tokio::main(flavor = "current_thread")]
// #[tokio::main]
async fn main() {
    acc().await
}

async fn acc() {
    for count in 0..1 { // 0到4（不包含5）

        // 启动一个异步任务
        tokio::spawn(async move{
            // tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            // println!("当前计数: {}", count);
            accept(count).await;
        });
    }
    // sleep(Duration::from_secs(15));
    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    println!("Exiting...");
}

async fn accept(index: i16) {

    // 连接到指定的地址和端口
    let socket_result = TcpStream::connect("192.168.10.104:3435").await;
    if let Err(e) = socket_result {
        eprintln!("链接失败{}",e);
        return
    }
    let mut socket = socket_result.unwrap();
    // println!("{}: Connected to the server!", index);
    for _ in 0 .. 10000 {
        let msg = index.to_string() + ": Hello from client!";

        // 要发送的消息
        let msg_byte_array = msg.as_bytes();

        // 发送消息到服务器
        socket.write_all(msg_byte_array).await.expect("TODO: panic message");
        socket.flush().await.expect("TODO: panic message");

        // // 创建一个缓冲区来读取响应
        // let mut buffer = [0; 512];
        // let len_result = socket.read(&mut buffer).await;
        // if let Err(e) = len_result{
        //     eprintln!("读物数据失败: {}", e);
        //     break;
        // }
        // let len = len_result.unwrap();
        //
        // // 打印服务器的响应
        // println!("{}: Received: {}", index, String::from_utf8_lossy(&buffer[..len]));
        tokio::time::sleep(tokio::time::Duration::from_secs_f32(1.0)).await;
        //sleep(Duration::from_secs(1));
    }
    let shutdown_result = socket.shutdown().await;
    if let Err(e) = shutdown_result{
        eprintln!("关闭失败: {}", e);
    }
}
