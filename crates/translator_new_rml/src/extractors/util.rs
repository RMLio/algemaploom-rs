use std::collections::HashSet;

use lazy_static::lazy_static;
use sophia_term::RcTerm;
use vocab::ToString;

/// Returns the lexical string representation of the given RDF term.
///
/// # Panics
///
/// Panics if the given term is of kind "Varialbe" or "Triple".
pub fn rcterm_to_string(rcterm: &RcTerm) -> String {
    let re_opt = match rcterm {
        RcTerm::Iri(iri_ref) => Some(iri_ref.to_string()),
        RcTerm::BlankNode(bnode_id) => Some(bnode_id.to_string()),
        RcTerm::Literal(generic_literal) => {
            Some(generic_literal.get_lexical_form().to_string())
        }
        _ => None,
    };

    re_opt.unwrap()
}

lazy_static! {
    static ref NUMBER_IRIS: HashSet<String> = HashSet::from([
        vocab::xsd::r#type::XSD_POSITIVE_INTEGER.to_string(),
        vocab::xsd::r#type::XSD_INT.to_string(),
        vocab::xsd::r#type::XSD_INTEGER.to_string(),
        vocab::xsd::r#type::XSD_LONG.to_string(),
        vocab::xsd::r#type::XSD_DOUBLE.to_string(),
    ]);
}
