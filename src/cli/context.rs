use super::OutputFormat;
use crate::sdk::Client;
use serde::Serialize;
use std::fmt::Debug;

pub struct Context {
    pub client: Client,
    pub fmt: OutputFormat,
}

impl Context {
    pub(crate) fn output<T>(&self, object: T)
    where
        T: Serialize + Debug,
    {
        match self.fmt {
            OutputFormat::Debug => println!("{:?}", object),
            OutputFormat::JSON => println!("{}", &serde_json::to_string(&object).unwrap()),
        }
    }
}
