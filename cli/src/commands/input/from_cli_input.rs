use anyhow::Result;

pub trait FromCliInput {
    fn from_cli_input() -> Result<Self>
    where
        Self: Sized;
}

pub trait FromCliInputWithContext {
    fn from_cli_input(existing_packs: &[&str]) -> Result<Self>
    where
        Self: Sized;
}
