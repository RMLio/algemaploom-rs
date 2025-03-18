use sophia_term::iri::Iri;
use sophia_term::Term;

pub mod extractors;
pub mod rml_model;

type TermString = Term<String>;
type IriString = Iri<String>;
