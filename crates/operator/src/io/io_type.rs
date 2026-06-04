use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum IOType {
    StdIn,
    StdOut,
    File,
    Kafka,
    Websocket,
    RDB,
    SPARQLEndpoint,
}

impl Default for IOType {
    fn default() -> Self {
        Self::StdOut
    }
}