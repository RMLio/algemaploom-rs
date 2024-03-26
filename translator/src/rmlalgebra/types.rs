use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use operator::Target;
use plangenerator::plan::{Plan, Processed};
use rml_interpreter::rml_model::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use rml_interpreter::rml_model::{PredicateObjectMap, TriplesMap};

#[derive(Debug, Clone)]
pub struct RefPOM<'a> {
    pub pms: Vec<&'a PredicateMap>,
    pub oms: Vec<&'a ObjectMap>,
}

impl<'a> PartialEq for RefPOM<'a> {
    fn eq(&self, other: &Self) -> bool {
        let pm_identifiers_left: Vec<_> =
            self.pms.iter().map(|pm| &pm.tm_info.identifier).collect();
        let pm_identifiers_right: Vec<_> =
            other.pms.iter().map(|pm| &pm.tm_info.identifier).collect();
        let om_identifiers_left: Vec<_> =
            self.oms.iter().map(|om| &om.tm_info.identifier).collect();
        let om_identifiers_right: Vec<_> =
            other.oms.iter().map(|om| &om.tm_info.identifier).collect();
        pm_identifiers_left == pm_identifiers_right
            && om_identifiers_left == om_identifiers_right
    }
}

impl<'a> From<&'a PredicateObjectMap> for RefPOM<'a> {
    fn from(value: &'a PredicateObjectMap) -> Self {
        Self {
            pms: value.predicate_maps.iter().collect(),
            oms: value.object_maps.iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Quad<'a> {
    pub triple: Triple<'a>,
    pub gm_opt: Option<&'a GraphMap>,
}

//Marker trait Eq to enable usage of Quads in set operations
impl<'a> Eq for Quad<'a> {}

impl<'a> PartialEq for Quad<'a> {
    fn eq(&self, other: &Self) -> bool {
        let gm_identifiers_left: Vec<_> = self
            .gm_opt
            .iter()
            .map(|gm| &gm.tm_info.identifier)
            .collect();
        let gm_identifiers_right: Vec<_> = other
            .gm_opt
            .iter()
            .map(|gm| &gm.tm_info.identifier)
            .collect();

        self.triple == other.triple
            && gm_identifiers_left == gm_identifiers_right
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Triple<'a> {
    pub sm: &'a SubjectMap,
    pub pm: &'a PredicateMap,
    pub om: &'a ObjectMap,
}

impl<'a> From<&'a Triple<'a>> for Triple<'a> {
    fn from(value: &'a Triple) -> Self {
        Triple {
            sm: value.sm,
            pm: value.pm,
            om: value.om,
        }
    }
}

impl<'a> PartialEq for Triple<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.sm.tm_info.identifier == other.sm.tm_info.identifier
            && self.pm.tm_info.identifier == other.pm.tm_info.identifier
            && self.om.tm_info.identifier == other.om.tm_info.identifier
    }
}

pub type TMPlanPair<'a> = (&'a TriplesMap, Rc<RefCell<Plan<Processed>>>);
#[derive(Debug, Clone)]
pub struct SearchMap<'a> {
    pub tm_rccellplan_map: HashMap<String, TMPlanPair<'a>>,
    pub variable_map:      HashMap<String, String>,
    pub target_map:        HashMap<String, Target>,
    pub lt_id_quad_map:    HashMap<String, HashSet<Quad<'a>>>,
}
