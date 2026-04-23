use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Get(String),
    Set(String, String),
    Del(String),
}

const GET_MIN_ARGS: usize = 2;
const GET_MAX_ARGS: usize = 2;

const SET_MIN_ARGS: usize = 3;

const DEL_MIN_ARGS: usize = 2;
const DEL_MAX_ARGS: usize = 2;

impl Request {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err("Invalid request".into());
        }

        match parts[0].to_uppercase().as_str() {
            "GET" => Request::get_callback(parts),
            "SET" => Request::set_callback(parts),
            "DEL" => Request::del_callback(parts),
            _ => Err("Unknown command".to_string()),
        }
    }

    pub fn get_callback(parts: Vec<&str>) -> Result<Self, String> {
        if parts.len() < GET_MIN_ARGS {
            Err("Invalid GET request. Too few arguments".to_string())
        } else if parts.len() > GET_MAX_ARGS {
            Err("Invalid GET request. Too many arguments".to_string())
        } else {
            Ok(Request::Get(parts[1].to_string()))
        }
    }

    pub fn set_callback(parts: Vec<&str>) -> Result<Self, String> {
        if parts.len() < SET_MIN_ARGS {
            Err("Invalid SET request. Too few arguments".to_string())
        } else {
            let value = parts[2..].join(" ");

            Ok(Request::Set(parts[1].to_string(), value))
        }
    }

    pub fn del_callback(parts: Vec<&str>) -> Result<Self, String> {
        if parts.len() < DEL_MIN_ARGS {
            Err("Invalid DEL request. Too few arguments".to_string())
        } else if parts.len() > DEL_MAX_ARGS {
            Err("Invalid DEL request. Too many arguments".to_string())
        } else {
            Ok(Request::Del(parts[1].to_string()))
        }
    }
}
