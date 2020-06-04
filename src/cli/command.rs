use super::router::RouteSegment;
use super::Context;
use async_trait::async_trait;
use clap::{ArgMatches};

// pub type Cmd = dyn Fn(&mut Context, ArgMatches<'a>) -> Result<(), Box<dyn std::error::Error>>;
pub type Cmd<'a, 'b> = Box<dyn Command<'a, 'b>>;

pub struct Arg<'a, 'b> {
    name: &'a str,
    help: Option<&'b str>,
}

#[async_trait]
pub trait Command<'a, 'b>: std::fmt::Debug {
    fn route(&self) -> Vec<RouteSegment>;
    fn args(&self) -> Vec<Arg<'a, 'b>>;
    async fn exec(
        &self,
        ctx: &mut Context,
        args: ArgMatches<'a>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
