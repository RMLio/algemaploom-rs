pub mod core;
pub mod fnml;
pub mod io;
pub mod lv;

pub trait AttributeAliaser {
    fn alias_attribute(&self, alias: &str) -> Self;
}
