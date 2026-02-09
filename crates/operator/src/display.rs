use anyhow::Result;
pub trait PrettyDisplay {
    fn pretty_string(&self) -> Result<String>;
}

pub trait JsonDisplay {
    fn json_string(&self) -> Result<String>;
}
