use std::fmt::{Display, Formatter, Result};
use snafu::prelude::*;

#[derive(Debug , Snafu)]
pub enum Errors {
    #[snafu(display("No ready function found! Add a new ready function by calling the `.set_ready()` function"))]
    NoStartFunctionFound,

    #[snafu(display("No ready function found! Add a new ready function by calling the `.set_ready()` function"))]
    NoExecuteFunctionFound
}

