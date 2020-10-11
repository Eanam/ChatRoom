/**
*聊天室的客户端
*/

use std::net::TcpStream;
use std::sync::mpsc::{self,TryRecvError};
use std::io::{self,ErrorKind,Read,Write};
use std::time::Duration;
use std::str;
use std::thread;
mod utils;

use chrono::prelude::*;

extern crate chrono;


const LOCAL_HOST : &str = "127.0.0.1:8080";

const MESSAGE_SIZE : usize = 1024;


fn main() {
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Failed to connect");

    client.set_nonblocking(true).expect("Failed to initiate non-blocking");

    let (sender,receiver) = mpsc::channel::<utils::TextMessage>();

    thread::spawn(move || loop{
        let mut buffer = vec![0;MESSAGE_SIZE];
        match client.read_exact(&mut buffer){
            Ok(_) =>{
                let message = buffer.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                let message_string = str::from_utf8(&message).unwrap();
                match message_string.parse::<utils::TextMessage>(){
                    Ok(text_message) => {
                        println!("--{}({}): {}",text_message.from,text_message.m_date,text_message.content);
                    }
                    Err(_) => {
                        println!("Error happen when message transform");
                    }
                }
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) =>{
                println!("Connection with server was served");
                break;
            }
        }
        match receiver.try_recv(){
            Ok(message) =>{
                let mut buffer = message.clone().to_string().into_bytes();
                buffer.resize(MESSAGE_SIZE,0);
                client.write_all(&buffer).expect("Writing to socket failed");
            },

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("*********************************");
    println!("************ WELCOME ************");
    println!("*********************************");

    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");
        let message = buffer.trim().to_string();
        let msg = message.clone();

        if message == "exit"{
            break;
        }

        //create a object of message
        let text_message = utils::TextMessage{
            from : String::from("send_address"),
            to : String::from("receive_address"),
            content : message.clone(),
            m_date : Utc::now().to_string()
        };

        if sender.send(text_message).is_err(){
            break;
        }

        println!("Message [{}] is sent",msg);
    }


    println!("*********************************");
    println!("************ GOODBYE ************");
    println!("*********************************");
}
