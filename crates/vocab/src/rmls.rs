pub const IRI: &str = "http://semweb.mmlab.be/ns/rmls#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const KAFKASTREAM: PAIR = (IRI, "KafkaStream");
    pub const TCPSOCKETSTREAM: PAIR = (IRI, "TcpSocketStream");
    pub const HTML: PAIR = (IRI, "HTML");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    // KAFKA PROPERTIES
    pub const TOPIC: PAIR = (IRI, "topic");
    pub const GROUPID: PAIR = (IRI, "groupId");
    pub const BROKER: PAIR = (IRI, "broker");

    // TCP PROPERTIES
    pub const HOSTNAME: PAIR = (IRI, "hostName");
    pub const PORT: PAIR = (IRI, "port");
}