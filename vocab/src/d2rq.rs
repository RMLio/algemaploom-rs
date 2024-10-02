pub const PREFIX: &str = "d2rq";
pub const IRI: &str = "http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;
    pub const DATABASE: PAIR = (IRI, "database");
    pub const SQLQUERY: PAIR = (IRI, "sqlQuery");
    pub const TABLENAME: PAIR = (IRI, "tableName");
    pub const COLUMN: PAIR = (IRI, "column");
    pub const USERNAME : PAIR = (IRI, "username");
    pub const PASSWORD : PAIR = (IRI, "password");
    pub const JDBCDSN : PAIR = (IRI, "jdbcDSN");
    pub const JDBCDriver : PAIR = (IRI, "jdbcDriver");

}