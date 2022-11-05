use crate::Errors;

pub struct Takoyaki<'a> {
    name: &'a str,
    ready: Option<Box<dyn Fn()>>,
    execute: Option<Box<dyn Fn()>>,
}

impl<'a> Takoyaki<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            ready: None,
            execute: None
        }
    }

    pub fn set_ready(&mut self , handler: Box<dyn Fn()>) {
        self.ready = Some(handler)
    }

    pub fn set_execute(&mut self , handler: Box<dyn Fn()>) {
        self.execute = Some(handler)
    }

    pub fn start(&self) -> Result<() , Errors> {
        // Prechecks
        if self.ready.is_none() {
            return Err(Errors::NoStartFunctionFound)
        }

        if self.execute.is_none() {
            return Err(Errors::NoExecuteFunctionFound)
        }

        Ok(())
    }
}
