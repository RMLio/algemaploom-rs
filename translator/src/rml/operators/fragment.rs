use std::collections::{HashMap, HashSet};

use operator::Fragmenter;

use crate::rml::types::Quad;
use crate::OperatorTranslator;
#[derive(Debug, Clone)]
pub struct FragmentTranslator<'a> {
    pub lt_quads_map: &'a HashMap<String, HashSet<Quad<'a>>>,
}

impl<'a> OperatorTranslator<Option<Fragmenter>> for FragmentTranslator<'a> {
    fn translate(&self) -> Option<Fragmenter> {
        translate_fragment_op_from_lts(self.lt_quads_map)
    }
}

fn translate_fragment_op_from_lts_str(
    lt_quads_map: &HashMap<String, HashSet<Quad>>,
    from_fragment: &str,
) -> Option<Fragmenter> {
    let target_lt_ids = lt_quads_map.keys();

    let to: Vec<String> = target_lt_ids.cloned().collect();

    if to.len() == 1 && to.first() == Some(&from_fragment.to_string()) {
        return None;
    }

    Some(Fragmenter {
        from: from_fragment.to_string(),
        to,
    })
}

fn translate_fragment_op_from_lts(
    lt_quads_map: &HashMap<String, HashSet<Quad>>,
) -> Option<Fragmenter> {
    translate_fragment_op_from_lts_str(lt_quads_map, "default")
}
