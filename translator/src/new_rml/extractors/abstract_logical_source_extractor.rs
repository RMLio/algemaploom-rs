use log::debug;
use sophia_api::term::Term;
use sophia_inmem::graph::FastGraph;

use super::store::get_object;
use super::Extractor;
use crate::new_rml::extractors::FromVocab;
use crate::new_rml::rml_model::v2::core::AbstractLogicalSourceEnum;
use crate::new_rml::rml_model::v2::io::source::LogicalSource;
use crate::new_rml::rml_model::v2::lv::LogicalView;

fn is_logical_view<TTerm>(term: TTerm, graph: &FastGraph) -> bool
where
    TTerm: Term,
{
    let type_term_opt = get_object(
        graph,
        term.borrow_term(),
        vocab::rdf::PROPERTY::TYPE.to_rcterm(),
    )
    .ok();

    debug!("Source {:?} has type {:?}", term, type_term_opt);
    if let Some(type_term) = type_term_opt {
        type_term == vocab::rml_lv::CLASS::LOGICAL_VIEW.to_rcterm()
    } else {
        get_object(graph, term, vocab::rml_lv::PROPERTY::VIEW_ON.to_rcterm())
            .is_ok()
    }
}

impl Extractor<AbstractLogicalSourceEnum> for AbstractLogicalSourceEnum {
    fn extract_self<TTerm>(
        subject: TTerm,
        graph: &FastGraph,
    ) -> super::ExtractorResult<AbstractLogicalSourceEnum>
    where
        TTerm: Term + Clone,
    {
        match is_logical_view(subject.borrow_term(), graph) {
            true => {
                let logical_view =
                    LogicalView::extract_self(subject.borrow_term(), graph)?;
                Ok(AbstractLogicalSourceEnum::LogicalView(logical_view))
            }
            false => {
                let logical_source =
                    LogicalSource::extract_self(subject.borrow_term(), graph)?;
                Ok(AbstractLogicalSourceEnum::LogicalSource(logical_source))
            }
        }
    }
}
