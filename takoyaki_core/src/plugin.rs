use crate::{ReadyState, PrintableGrid, Cache};

pub trait Plugin<'a , T , U> {
    fn new() -> Self where Self: Sized;

    fn name(&self) -> &'a str;

    fn ready(&self , config: U , cache: Cache) -> ReadyState;

    fn execute(&self , data: T) -> PrintableGrid;
}

