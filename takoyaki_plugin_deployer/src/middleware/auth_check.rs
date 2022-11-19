use std::net::SocketAddr;

use crate::{middleware::Middleware , STATE};

pub struct AuthCheck {

}

impl Middleware for AuthCheck {
    fn new() -> Self {
        Self {

        }
    }

    fn execute(&self , message: &str , incoming_addr: SocketAddr) -> bool {
        if message.trim().starts_with("/auth") {
            return true
        }

        STATE.lock().exists(&incoming_addr)
    }
}
