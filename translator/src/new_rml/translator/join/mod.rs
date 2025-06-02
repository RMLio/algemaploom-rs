use plan::states::join::join;

use super::OperatorTranslator;
use crate::new_rml::error::NewRMLTranslationResult;
use crate::new_rml::extractors::error::ParseError;
use crate::new_rml::rml_model::v2::core::TriplesMap;

#[derive(Debug, Clone)]
pub struct JoinTranslator {}

impl OperatorTranslator for JoinTranslator {
    type Input = TriplesMap;

    type Output = ();

    fn translate_with_store(
        store: &super::store::SearchStore,
        child_trip_map: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let parent_tms_joins = child_trip_map.get_parent_triples_maps_ids();
        let child_logical_source_id =
            child_trip_map.abs_logical_source.get_identifier();

        let child_plan = store
            .ls_id_sourced_plan_map
            .get(&child_logical_source_id)
            .ok_or(ParseError::GenericError(format!(
                "Search store cannot found the associated plan for the logical source id: {:?}",
                child_logical_source_id
            )))?;

        for (parent_tm_id, join_conditions) in parent_tms_joins {
            let parent_tm =
                store.tm_search_map.get(&parent_tm_id).unwrap_or_else(|| {
                    panic!(
                        "Given triples map id {:?} does not exist!",
                        parent_tm_id
                    )
                });

            let parent_logical_source_id =
                parent_tm.abs_logical_source.get_identifier();

            let parent_plan = store
                    .ls_id_sourced_plan_map
                    .get(&parent_logical_source_id)
                    .ok_or(ParseError::GenericError(format!(
                        "Search store cannot found the associated plan for the logical source id: {:?}",
                        child_logical_source_id
                    )))?;
            let child_attributes = join_conditions
                .iter()
                .flat_map(|jc| jc.child.get_value())
                .collect();
            let parent_attributes = join_conditions
                .iter()
                .flat_map(|jc| jc.parent.get_value())
                .collect();

            // Join the plans and progress the cursur
            let _ = join(child_plan.clone(), parent_plan.clone())?
                .alias("inner_join")?
                .where_by(child_attributes)?
                .compared_to(parent_attributes)?;
        }
        Ok(())
    }
}
