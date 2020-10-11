/**
*聊天室的服务端
**/

use std::sync::mpsc;
use std::thread;
use std::net::TcpListener;
use std::io::{ErrorKind,Read,Write};
mod utils;


//监听的地址和端口
const LOCAL_HOST : &str = "127.0.0.1:8080";

//信息的缓存区大小
const MESSAGE_SIZE : usize = 1024;

//休眠功能：使得我们的线程在每一个循环中间休眠一段时间
fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}



fn main() {
    //实例化监听器
    let listener = TcpListener::bind(LOCAL_HOST).expect("Create TcpListener failed");
    //将监听器设置为非阻塞模式
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    //创建一个动态数组来存放
    let mut clients = vec![];

    //实例化信道，并将信道传输的数据类型设置为String
    let (sender,receiver) = mpsc::channel::<String>();

    loop{
        //解析监听器监听到的结果，主要是TCP流和Socket地址
        if let Ok((mut socket,address)) = listener.accept(){
            println!("Client {} : CONNECTED",address);

            let sender = sender.clone();
            clients.push(socket.try_clone().expect("Failed to clone Client"));

            thread::spawn(move || loop{
                let mut buffer = vec![0;MESSAGE_SIZE];
                match socket.read_exact(&mut buffer){
                    Ok(_)=>{
                        let message = buffer.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                        let message = String::from_utf8(message).expect("Invalid utf8 message");
                        sender.send(message).expect("Fail to send message to receiver");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}",address);
                        break;
                    }
                }

                sleep();
            });
        }
        // match receiver.try_recv(){
        //     Ok(Message) =>{
        //         let msg = Message.clone();
        //         println!("Message [{}] is received",msg);  
        //     },
        //     Err(err) =>{
        //         println!("{} happened when receive message",err);
        //     }
        // }
        if let Ok(message) = receiver.try_recv(){
            let msg = message.clone();
            println!("Message [{:#?}] is received",msg);
            clients = clients.into_iter().filter_map(|mut client|{
                let mut buffer = message.clone().into_bytes();

                buffer.resize(MESSAGE_SIZE,0);

                client.write_all(&buffer).map(|_| client).ok()
            }).collect::<Vec<_>>();

        }
        sleep();
    }
}
