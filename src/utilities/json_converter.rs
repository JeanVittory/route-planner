use serde::Serialize;
use serde_json::to_string;

pub fn get_json_format<T:Serialize>(payload: &T) -> String{
    match to_string(payload){
        Ok(value) => value,
        Err(error) =>{
            eprint!("Error: {}", error);
            String::new()
        }
    }
}