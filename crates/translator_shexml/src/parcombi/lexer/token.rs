#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShExMLToken {
    /// prologue
    Prefix,
    Source,
    Iterator,
    Matcher,
    Expression,
    AutoIncrement,
    Function,

    /// Source type
    File,
    HTTP,
    HTTPS,
    JDBC(String),

    /// interim
    As,
    Matching,
    Field,
    PushField,
    PopField,
    Union,
    Join,
    StringSep(String),
    If,

    /// function language
    FunctionLang(String),

    /// prefix autoinc
    AutoIncPrefix(String),
    AutoIncStart(u32),
    AutoIncEnd(u32),
    AutoIncStep(u32),
    AutoIncSuffix(String),

    /// "jsonpath:" "csvperrow" "xmlpath:" etc..
    IteratorType(String),
    IteratorQuery(String),

    /// Baseprefix
    BasePrefix,
    /// prefix namespace
    PrefixNS(String),
    /// prefix local name
    PrefixLN(String),

    URI(String),

    /// Identifier used by source, matcher, function, expression
    Ident(String),


    /// class type a
    Type,

    /// values used in matcher
    Value(String),
    /// Field query
    FieldQuery(String),

    /// :
    PrefixSep,

    /// <
    AngleStart,
    /// >
    AngleEnd,

    /// {
    CurlStart,
    /// }
    CurlEnd,

    /// ;
    PredicateSplit,
    /// ,
    ObjectSplit,

    /// ,
    Comma,

    /// .
    Dot,

    /// &
    MatcherSplit,

    /// @
    AtSymb,

    //
    ShapeNode {
        prefix: String,
        local:  String,
    },
    ShapeTerm {
        prefix: String,
        local:  String,
    },
    LangTag(String),

    /// (
    BrackStart,
    /// )
    BrackEnd,

    /// [
    SqBrackStart,
    /// ]
    SqBrackEnd,
}
