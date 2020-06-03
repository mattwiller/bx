use super::router::RouteSegment;
use super::Context;
use async_trait::async_trait;
use clap::ArgMatches;

// pub type Cmd = dyn Fn(&mut Context, ArgMatches<'a>) -> Result<(), Box<dyn std::error::Error>>;
pub type Cmd<'a> = Box<dyn Command<'a>>;

#[async_trait]
pub trait Command<'a> {
    fn route(&self) -> Vec<RouteSegment>;
    async fn exec(
        &self,
        ctx: &mut Context,
        args: ArgMatches<'a>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
