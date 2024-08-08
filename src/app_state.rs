use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
    pub version: String,
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>,
}