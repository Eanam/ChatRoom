use std::string::String;
use std::fmt::Debug;

use chrono::prelude::*;

extern crate chrono;

use std::string::ToString;

use std::str::FromStr;
use std::num::ParseIntError;

use std::time::Duration;
use chrono::Duration as Durationx;

#[derive(Debug)]
struct TextMessage{
    from : String,//the address of sender
    to : String,//the address of receiver
    content : String,//the content of msg
    time : i64
}

//the mothod convert TextMessage to String 
impl ToString for TextMessage{
    fn to_string(&self) -> String{
        format!("({},{},{},{})",self.from,self.to,self.content,self.time)
    }
}

impl FromStr for TextMessage{
    type Err = ParseIntError;
    fn from_str(s : &str) -> Result<Self,Self::Err>{
        let message_info : Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' ).split(",").collect();
        let from = message_info[0].to_string();
        let to = message_info[1].to_string();
        let content = message_info[2].to_string();
        let time = message_info[3].parse::<i64>()?;
        Ok(TextMessage{from,to,content,time})
    }
}



fn main() {
    println!("Hello, world!");
    println!("Creating first textmessage...");
    let firstMessage = TextMessage{
        from : String::from("eanam1"),
        to : String::from("eanam2"),
        content : String::from("can you hear from me"),
        time : Local::now().timestamp_millis()
    };

    let message_string = firstMessage.to_string();

    println!("{}",message_string);
    // println!("{:#?}",firstMessage);

    // println!("********************");
    // let local = Local::now();
    // println!("{}",local.timestamp_millis());

    let second_message = message_string.parse::<TextMessage>().unwrap();

    println!("second message is {:#?}",second_message);

    // let x = (Durationx::from_std(Duration::from_millis(second_message.time as u64)).unwrap())-Local.ymd(1970.1.1);

    // println!("-----{}",);

    let x = Utc::now();
    println!("{:#?}",x.to_string());
}   
