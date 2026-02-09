use std::collections::HashSet;

use lazy_static::lazy_static;
use vocab::ToString;

lazy_static! {
    static ref NUMBER_IRIS: HashSet<String> = HashSet::from([
        vocab::xsd::TYPE::XSD_POSITIVE_INTEGER.to_string(),
        vocab::xsd::TYPE::XSD_INT.to_string(),
        vocab::xsd::TYPE::XSD_INTEGER.to_string(),
        vocab::xsd::TYPE::XSD_LONG.to_string(),
        vocab::xsd::TYPE::XSD_DOUBLE.to_string(),
    ]);
}
