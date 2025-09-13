use anyhow::Result;

pub trait FromCliInput {
    fn from_cli_input() -> Result<Self> where Self: Sized;
}