pub const PREFIX: &str = "csvw";
pub const IRI: &str = "http://www.w3.org/ns/csvw#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const TABLE: PAIR = (IRI, "Table");
    pub const DIALECT: PAIR = (IRI, "Dialect");
    pub const SCHEMA: PAIR = (IRI, "Schema");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const DIALECT: PAIR = (IRI, "dialect");
    pub const URL: PAIR = (IRI, "url");

    // Parse configs properties
    pub const TRIM: PAIR = (IRI, "trim");
    pub const COMMENT_PREFIX: PAIR = (IRI, "commentPrefix");
    pub const DELIMITER: PAIR = (IRI, "delimiter");
    pub const DOUBLE_QUOTE: PAIR = (IRI, "doubleQuote");
    pub const ENCODING: PAIR = (IRI, "encoding");
    pub const HEADER: PAIR = (IRI, "header");
    pub const HEADER_ROW_COUNT: PAIR = (IRI, "headerRowCount");
    pub const LINE_TERMINATORS: PAIR = (IRI, "lineTerminators");
    pub const QUOTE_CHARS: PAIR = (IRI, "quoteChars");
    pub const SKIP_BLANK_ROWS: PAIR = (IRI, "skipBlankRows");
    pub const SKIP_COLUMNS: PAIR = (IRI, "skipColumns");
    pub const SKIP_ROWS: PAIR = (IRI, "skipRows");
    pub const SKIP_INITIAL_SPACE: PAIR = (IRI, "skipInitialSpace");
}
