// Web of Things (WoT) Thing Description vocabulary
// https://www.w3.org/TR/wot-thing-description/

pub const PREFIX: &str = "td";
pub const IRI: &str = "https://www.w3.org/2019/wot/td#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    
    pub const THING: PAIR = (IRI, "Thing");
    pub const PROPERTY_AFFORDANCE: PAIR = (IRI, "PropertyAffordance");
    pub const ACTION_AFFORDANCE: PAIR = (IRI, "ActionAffordance");
    pub const EVENT_AFFORDANCE: PAIR = (IRI, "EventAffordance");
    pub const INTERACTION_AFFORDANCE: PAIR = (IRI, "InteractionAffordance");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;
    
    pub const HAS_PROPERTY_AFFORDANCE: PAIR = (IRI, "hasPropertyAffordance");
    pub const HAS_ACTION_AFFORDANCE: PAIR = (IRI, "hasActionAffordance");
    pub const HAS_EVENT_AFFORDANCE: PAIR = (IRI, "hasEventAffordance");
    pub const HAS_FORM: PAIR = (IRI, "hasForm");
    pub const NAME: PAIR = (IRI, "name");
    pub const TITLE: PAIR = (IRI, "title");
    pub const DESCRIPTION: PAIR = (IRI, "description");
}
