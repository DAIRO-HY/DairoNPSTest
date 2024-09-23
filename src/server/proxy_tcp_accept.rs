use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

///等待客户端连接
pub async fn start() {

    // 绑定到指定地址和端口
    let bind_result = TcpListener::bind("0.0.0.0:1680").await;
    if let Err(e) = bind_result {
        eprintln!("创建TcpListener失败: {}", e);
        return;
    }
    println!("Server listening on port 1680");
    let listener = bind_result.unwrap();
    loop {
        let accept_result = listener.accept().await;
        if let Err(e) = accept_result {
            eprintln!("接受连接失败: {}", e);
            break;
        }
        let (socket, _) = accept_result.unwrap();
        println!("New connection: {:?}", socket.peer_addr());

        //开启异步任务
        tokio::spawn(async {
            handle(socket).await
        });
    }
}

///等待客户端连接
async fn handle(mut socket: TcpStream) {
    loop {

        // 创建一个缓冲区来读取数据
        let mut buffer = [0; 8 * 1024];

        // 从客户端读取数据
        let len_result = socket.read(&mut buffer).await;
        if let Err(e) = len_result {
            eprintln!("读取数据失败: {}", e);
            break;
        }
        let len = len_result.unwrap();
        if len == 0{//客户端已经关闭了
            break;
        }

        // 打印接收到的数据
        println!("Received: {}", String::from_utf8_lossy(&buffer[..len]));

        tokio::time::sleep(tokio::time::Duration::from_secs_f32(5.0)).await;

        // 回应客户端
        let write_result = socket.write(b"Hello from server!").await;
        if let Err(e) = write_result {
            eprintln!("写入数据失败: {}", e);
            break;
        }
    }
}