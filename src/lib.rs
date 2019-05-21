use chrono::prelude::*;
use chrono::{Duration};
use std::collections::HashMap;

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
 pub flex_type:HashMap<String,String>,

}
pub fn lvl_to_string(input:&Level)-> String{
        match input {
            Level::Error => {
                "error".to_string()
            },
            Level::Warn => {
                "error".to_string()
            },
            Level::Info => {
                "error".to_string()
            },
            Level::Debug => {
                "error".to_string()
            }
        }
    }

impl LogObject {

    /*
        Contructor that initialises al variables to empty string and standard error
    */

    pub fn new() -> LogObject {
        let now_utc: DateTime<Utc> = Utc::now();
        
        LogObject{timestamp:now_utc.to_rfc3339_opts(SecondsFormat::Secs, true),msg:"".to_string(),lvl:Level::Error,op:"".to_string(),flex_type:HashMap::new()}

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
    pub fn optional_type(mut self,t:String,t2:String)-> LogObject{
        self.flex_type.insert(
            t,
            t2,
        );
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
    pub fn to_string(mut self)->String {
        //let mut flex_type = HashMap::new();
        self.flex_type.insert(
            "lvl".to_string(),
            lvl_to_string(&self.lvl.clone()),
        );
        self.flex_type.insert(
            "msg".to_string(),
            self.msg.clone(),
        );
        self.flex_type.insert(
            "op".to_string(),
            self.op.clone(),
        );
        self.flex_type.insert(
            "@timestamp".to_string(),
            self.timestamp.clone(),
        );
     
        let retval = serde_json::to_string(&self.flex_type.clone()).unwrap();
        retval
    }

    /*
        outputs a string form of log object with carriage returns for readability
    */
    pub fn to_pretty_string(mut self)-> String {
         self.flex_type.insert(
            "lvl".to_string(),
            lvl_to_string(&self.lvl.clone()),
        );
        self.flex_type.insert(
            "msg".to_string(),
            self.msg.clone(),
        );
        self.flex_type.insert(
            "op".to_string(),
            self.op.clone(),
        );
        self.flex_type.insert(
            "@timestamp".to_string(),
            self.timestamp.clone(),
        );
        serde_json::to_string_pretty(&self.flex_type.clone()).unwrap()
    }

    pub fn print(self){
        println!("{}",self.to_string());
    }
    pub fn print_pretty(self){
        println!("{}",self.to_pretty_string());
    }
    pub fn map_test(self){
        let mut flex_type = HashMap::new();
        flex_type.insert(
            "lvl".to_string(),
            "Error".to_string(),
        );
        flex_type.insert(
            "msg".to_string(),
            "a blah blah".to_string(),
        );

        println!("{}", serde_json::to_string(&flex_type).unwrap());
    }
}
