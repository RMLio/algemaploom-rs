pub mod join;
pub mod processed;
pub mod serialized;
pub mod start;

#[derive(Debug, Clone)]
pub struct Init {}
#[derive(Debug, Clone)]
pub struct Processed {}
#[derive(Debug, Clone)]
pub struct Serialized {}
#[derive(Debug, Clone)]
pub struct Sunk {}
