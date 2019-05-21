use chrono::prelude::*;
use chrono::{Duration};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogObject {
#[serde(rename = "@timestamp")]
 pub timestamp:String,
 pub msg:String,
 pub lvl:Level,
 pub op:String,

}

impl LogObject {

    /*
        Contructor that initialises al variables to empty string and standard error
    */

    pub fn new() -> LogObject {
        let now_utc: DateTime<Utc> = Utc::now();
      
        LogObject{timestamp:now_utc.to_rfc3339_opts(SecondsFormat::Secs, true),msg:"".to_string(),lvl:Level::Error,op:"".to_string()}

    }
    pub fn msg(mut self,msg:String) -> LogObject {
        self.msg = msg;
        self
    }
    pub fn op(mut self,op:String) -> LogObject {
        self.op = op;
        self
    }

    /*
        Take in a string and sets the mathcing error enum. Case is ignored 
        Choices are:
        1.Error
        2.Warn
        3.Warning
        4.Info
        5.Debug
    */
    pub fn lvl(mut self,lvl:String) -> LogObject {
        
        let level:Level = match lvl.to_uppercase().as_ref() {
        "ERROR" => {
            Level::Error
        },
        "WARN" => {
            Level::Warn
        },
        "WARNING" => {
            Level::Warn
        },
        "INFO" => {
            Level::Info
        },
        "DEBUG" => {
            Level::Debug
        }
        _ =>{Level::Error}
    };
    
        self.lvl = level;
        self

    }


    // optional data to append to msg field. Use this for errors rust generates
    pub fn optionaldata(mut self,optm:String) ->LogObject {
         let stringbuilder = vec![self.msg.clone(),optm.clone()];
         self.msg = stringbuilder.join(":");
         self
    }

    /*
        Outputs a string form of log object
    */
    pub fn to_string(self)->String {
        serde_json::to_string(&self).unwrap()
    }

    /*
        outputs a string form of log object with carriage returns for readability
    */
    pub fn to_pretty_string(self)-> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn print(self){
        println!("{}",self.to_string());
    }
    pub fn print_pretty(self){
        println!("{}",self.to_pretty_string());
    }
}
