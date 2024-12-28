use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct ApplicationState{
    pub api_key: Arc<Mutex<String>>,
}