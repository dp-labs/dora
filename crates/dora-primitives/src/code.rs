use std::fmt::Debug;

pub trait Transaction: Debug {
    type Context;
    type Result;

    fn run(&self, ctx: &mut Self::Context, initial_gas: u64) -> Self::Result;
}
