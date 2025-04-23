use std::marker::PhantomData;

use operator::Join;
use plangenerator::states::join::join;

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
        if let Some(parent_trip_map_ids) =
            store.tm_id_join_map.get(&child_trip_map.identifier)
        {
            let child_logical_source_id =
                child_trip_map.abs_logical_source.get_identifier();
            let parent_trip_map_iter = parent_trip_map_ids
                .iter()
                .filter_map(|tm_id| store.tm_search_map.get(tm_id).copied())
                .filter(|tm| {
                    tm.abs_logical_source.get_identifier()
                        != child_logical_source_id
                });
            let child_plan = store
                .ls_id_sourced_plan_map
                .get(&child_logical_source_id)
                .ok_or(ParseError::GenericError(format!(
                    "Search store cannot found the associated plan for the logical source id: {:?}",
                    child_logical_source_id
                )))?
                ;

            for parent_trip_map in parent_trip_map_iter {
                let parent_logical_source_id =
                    parent_trip_map.abs_logical_source.get_identifier();
                let parent_plan = store
                    .ls_id_sourced_plan_map
                    .get(&parent_logical_source_id)
                    .ok_or(ParseError::GenericError(format!(
                        "Search store cannot found the associated plan for the logical source id: {:?}",
                        child_logical_source_id
                    )))?
                    ;


               // join(child_plan.clone(), parent_plan.clone())?
               //     .alias("inner_join")? 
               //     .where_by(attributes);
            }

            todo!()
        } else {
            Ok(())
        }
    }
}
