// 导入需要的依赖
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// 处理方法
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 缓冲数组
    let mut buf = [0; 512];
    loop {
        // 读取数据
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        // 打印输出
        println!("echo: {}", String::from_utf8_lossy(&buf[..]));
        // 返回数据
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() -> std::io::Result<()> {
    // 监听端口
    let listener = TcpListener::bind("127.0.0.1:8080");
    // 错误处理
    match listener {
        // 正确的时候的处理
        Ok(listener) => {
            println!("server run");
            // 线程数组
            let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
            // 处理连接
            for stream in listener.incoming() {
                let stream = stream.expect("failed!");
                // 一个链接一个线程
                let handle = thread::spawn(move || {
                    handle_client(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                });

                thread_vec.push(handle);
            }
            // 多线程方式运行
            for handle in thread_vec {
                handle.join().unwrap();
            }
        },
        // 错误的处理
        Err(err) => println!("error {:?}", err),

    }
    Ok(())
}