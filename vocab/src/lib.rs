pub mod comp;
pub mod csvw;
pub mod fnml;
pub mod fno;
pub mod formats;
pub mod query;
pub mod r2rml;
pub mod rdf;
pub mod rml;
pub mod rmlt;
pub mod void;
pub mod xsd;
pub mod rml_core;
pub mod d2rq;

pub type PAIR<'a> = (&'a str, &'a str);

pub trait ToString {
    fn to_string(self) -> String;
}

impl<'a> ToString for PAIR<'a> {
    fn to_string(self) -> String  {
        format!("{}{}", self.0, self.1)
    }
}
