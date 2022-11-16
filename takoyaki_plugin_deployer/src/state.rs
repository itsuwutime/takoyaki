use std::net::SocketAddr;

pub struct State {
    allowed_ip_list: Vec<SocketAddr>
}

impl State {
    pub fn new() -> Self {
        Self {
            allowed_ip_list: vec![]
        }
    }

    pub fn allow_ip(&mut self , ip: SocketAddr) {
        self.allowed_ip_list.push(ip)
    }

    pub fn is_allowed(&mut self , ip: SocketAddr) -> bool {
        self.allowed_ip_list.contains(&ip)
    }

    pub fn remove_ip(&mut self , ip: SocketAddr) -> Option<SocketAddr> {
        if !self.allowed_ip_list.contains(&ip) {
            return None
        }

        Some(self.allowed_ip_list.remove(
            self.allowed_ip_list.iter()
                .position(|x| *x == ip)
                .unwrap()
        ))
    }
}

