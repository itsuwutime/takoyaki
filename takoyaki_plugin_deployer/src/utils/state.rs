use std::net::SocketAddr;

pub struct State {
    authorized_ips: Vec<SocketAddr>
}

impl State {
    pub fn new() -> Self {
        Self {
            authorized_ips: vec![]
        }
    }

    pub fn authorize_ip(&mut self , ip: SocketAddr) {
        self.authorized_ips.push(ip);
    }

    pub fn remove_ip(&mut self , ip: SocketAddr) {
        self.authorized_ips.remove(
            self.authorized_ips.iter()
                .position(|x| *x == ip)
                .unwrap()
        );
    } 

    pub fn is_authorized(&self , ip: &SocketAddr) -> bool {
        self.authorized_ips.contains(ip)
    }
}