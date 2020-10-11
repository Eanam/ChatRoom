use std::string::String;
use std::fmt::Debug;
use std::clone::Clone;
use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;


//define the text message struct\
#[derive(Debug,Clone)]
pub struct TextMessage{
    pub from : String, //the address of sender
    pub to : String,//the address of receiver
    pub content : String,//the content of msg
    pub m_date : String //the date of the message(millis)
}

//the mothod to convert TextMessage to String 
impl ToString for TextMessage{
    fn to_string(&self) -> String{
        format!("({},{},{},{})",self.from,self.to,self.content,self.m_date)
    }
}

//the mothod to convert String to TextMessage
impl FromStr for TextMessage{
    type Err = ParseIntError;
    fn from_str(s : &str) -> Result<Self,Self::Err>{
        let message_info : Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' ).split(",").collect();
        let from = message_info[0].to_string();
        let to = message_info[1].to_string();
        let content = message_info[2].to_string();
        // let m_date = message_info[3].parse::<i64>()?;
        let m_date = message_info[3].to_string();
        Ok(TextMessage{from,to,content,m_date})
    }
}

//some tools
mod util{
    //millis to data(yyyy-mm-dd hh:mm:ss)
}
