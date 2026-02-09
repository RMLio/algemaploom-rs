pub const PREFIX: &str = "rml";

pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const VIEW_ON: PAIR = (IRI, "viewOn");
    pub const FIELD: PAIR = (IRI, "field");
    pub const FIELD_NAME: PAIR = (IRI, "fieldName");
    pub const PARENT_LOGICAL_VIEW: PAIR = (IRI, "parentLogicalView");

    pub const ON_FIELDS: PAIR = (IRI, "onFields");
    pub const TARGET_FIELDS: PAIR = (IRI, "targetFields");
    pub const LEFT_JOIN: PAIR = (IRI, "leftJoin");
    pub const INNER_JOIN: PAIR = (IRI, "innerJoin");
    pub const TARGET_VIEW: PAIR = (IRI, "targetView");
    pub const STRUCTURAL_ANNOTATION: PAIR = (IRI, "structuralAnnotation");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const LOGICAL_VIEW_JOIN: PAIR = (IRI, "LogicalViewJoin");
    pub const LOGICAL_VIEW: PAIR = (IRI, "LogicalView");

    pub const FIELD: PAIR = (IRI, "Field");
    pub const ITERABLE_FIELD: PAIR = (IRI, "IterableField");
    pub const EXRESSION_FIELD: PAIR = (IRI, "ExpressionField");

    pub const STRUCTURAL_ANNOTATION: PAIR = (IRI, "StructuralAnnotation");
    pub const FOREIGN_KEY_ANNOTATION: PAIR = (IRI, "ForeignKeyAnnotation");
    pub const IRI_SAFE_ANNOTATION: PAIR = (IRI, "IRISafeAnnotation");
    pub const NON_NULLABLE_ANNOTATION: PAIR = (IRI, "NonNullableAnnotation");
    pub const UNIQUE_ANNOTATION: PAIR = (IRI, "UniqueAnnotation");
    pub const PRIMARY_KEY_ANNOTATION: PAIR = (IRI, "PrimaryKeyAnnotation");
    pub const INCLUSION_DEPENDENCY_ANNOTATION: PAIR = (IRI, "InclusionDependencyAnnotation");
}
