use std::net::SocketAddr;

#[derive(Default)]
pub struct State {
    authorized_ips: Vec<SocketAddr>
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authorize_ip(&mut self , ip: SocketAddr) {
        self.authorized_ips.push(ip)
    }

    pub fn exists(&self , ip: &SocketAddr) -> bool {
        self.authorized_ips.contains(ip)
    }

    pub fn remove(&mut self , ip: SocketAddr) {
        if !self.exists(&ip) {
            return
        }

        self.authorized_ips.remove(
            self.authorized_ips.iter().position(|x| *x == ip).unwrap()
        );
    }
}
