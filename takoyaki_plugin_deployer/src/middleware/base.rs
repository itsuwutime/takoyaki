use std::net::SocketAddr;

pub trait Middleware {
    fn new() -> Self where Self: Sized;
    fn execute(&self , message: &str , incoming_addr: SocketAddr) -> bool;
}

#[derive(Default)]
pub struct MiddleWareManager {
    middlewares: Vec<Box<dyn Middleware + Send + Sync>>
}

impl MiddleWareManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware + Send + Sync>) {
        self.middlewares.push(middleware)
    }

    pub fn passes(&self , message: String , incoming_addr: SocketAddr) -> bool {
        let mut resp = vec![];

        for middleware in &self.middlewares {
            resp.push(middleware.execute(&message, incoming_addr))
        }

        resp.iter().all(|x| *x == true)
    }
}
