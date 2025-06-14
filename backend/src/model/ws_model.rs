use axum::extract::ws::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub type Tx = mpsc::UnboundedSender<Message>;
pub type UserSocketMap = Arc<Mutex<HashMap<String, Tx>>>;

#[derive(Clone)]
pub struct AppState {
    pub user_sockets: UserSocketMap,
}