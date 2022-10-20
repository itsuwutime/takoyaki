use crate::ready_state::ReadyState;

pub trait Plugin<'a , T> {
    fn new() -> Self where Self: Sized;

    fn name(&self) -> &'a str;

    fn ready(&self) -> ReadyState;

    fn execute(&self , data: T);
}

