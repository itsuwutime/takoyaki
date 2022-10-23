use crate::{ready_state::ReadyState, printable_grid::PrintableGrid};

pub trait Plugin<'a , T , U> {
    fn new() -> Self where Self: Sized;

    fn name(&self) -> &'a str;

    fn ready(&self , config: U) -> ReadyState;

    fn execute(&self , data: T) -> PrintableGrid;
}

