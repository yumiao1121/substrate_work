use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    let bytes_read = stream.read(&mut buf)?;//读取消息到buf中
    if bytes_read == 0 {
        return Ok(());
    }
    println!("Request:{}",String::from_utf8_lossy(&buf[..]));//打印接收到的TCP消息内容
    stream.write(&buf[..bytes_read])?; //将消息返回
    thread::sleep(time::Duration::from_secs(1 as u64)); //等待1S
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //监听端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); 

    for stream in listener.incoming() {
      
       
        let stream: TcpStream = match stream{
            Ok(stream) => stream,
            Err(_) => panic!("err"),
        };
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error)); //创建协程，并且作错误处理
        });

        thread_vec.push(handle); //加入协程管理数组
    }

    for handle in thread_vec {
        handle.join().unwrap(); //等待关联的线程完成
    }
    Ok(())
}
