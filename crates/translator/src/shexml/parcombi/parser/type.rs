use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShExMLError {
    ParseError(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShExMLDocument {
    pub prefixes:         Vec<Prefix>,
    pub sources:          Vec<Source>,
    pub iterators:        Vec<Iterator>,
    pub expression_stmts: Vec<ExpressionStmt>,
    pub auto_increments:  Vec<AutoIncrement>,
    pub functions:        Vec<Function>,
    pub matchers:         Vec<Matcher>,
    pub graph_shapes:     Vec<GraphShapes>,
}
impl ShExMLDocument {
    pub fn convert_to_indexed(self) -> IndexedShExMLDocument {
        let prefixes = self
            .prefixes
            .into_iter()
            .map(|pref| (pref.prefix.to_string(), pref))
            .collect();

        let sources = self
            .sources
            .into_iter()
            .map(|source| (source.ident.clone(), source))
            .collect();

        let iterators = self
            .iterators
            .into_iter()
            .map(|iter| (iter.ident.clone(), iter))
            .collect();

        let expression_stmts = self
            .expression_stmts
            .into_iter()
            .map(|expr| (expr.ident.clone(), expr))
            .collect();

        let auto_increments = self
            .auto_increments
            .into_iter()
            .map(|auto_inc| (auto_inc.ident.clone(), auto_inc))
            .collect();

        let functions = self
            .functions
            .into_iter()
            .map(|func| (func.ident.clone(), func))
            .collect();

        let matchers = self
            .matchers
            .into_iter()
            .map(|matcher| (matcher.ident.clone(), matcher))
            .collect();

        let graph_shapes = self
            .graph_shapes
            .clone()
            .into_iter()
            .map(|graph_shape| (graph_shape.ident.to_string(), graph_shape))
            .collect();

        let shapes = self
            .graph_shapes
            .into_iter()
            .map(|graph_shape| (graph_shape.ident, graph_shape.shapes))
            .flat_map(|(graph_ident, shape)| {
                shape.into_iter().map(move |s| (graph_ident.clone(), s))
            })
            .map(|(graph_ident, shape)| {
                ((graph_ident, shape.ident.clone()), shape)
            })
            .collect();
        IndexedShExMLDocument {
            prefixes,
            sources,
            iterators,
            expression_stmts,
            auto_increments,
            functions,
            matchers,
            graph_shapes,
            shapes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexedShExMLDocument {
    pub prefixes:         HashMap<String, Prefix>,
    pub sources:          HashMap<String, Source>,
    pub iterators:        HashMap<String, Iterator>,
    pub expression_stmts: HashMap<String, ExpressionStmt>,
    pub auto_increments:  HashMap<String, AutoIncrement>,
    pub functions:        HashMap<String, Function>,
    pub matchers:         HashMap<String, Matcher>,
    pub graph_shapes:     HashMap<String, GraphShapes>,
    pub shapes:           HashMap<(ShapeIdent, ShapeIdent), Shape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prefix {
    pub prefix: PrefixNameSpace,
    pub uri:    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Normal,
    Push,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub field_type: FieldType,
    pub ident:      String,
    pub query:      String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    File,
    JDBC(String),
    HTTP,
    HTTPS,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub ident:       String,
    pub source_type: SourceType,
    pub uri:         String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iterator {
    pub ident:           String,
    pub query:           Option<String>,
    pub iter_type:       Option<IteratorType>,
    pub fields:          Vec<Field>,
    pub nested_iterator: Vec<Iterator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IteratorType {
    JSONPath,
    XPath,
    CSVRows,
    SQL,
    SPARQL,
}

impl FromStr for IteratorType {
    type Err = ShExMLError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jsonpath:" => Ok(IteratorType::JSONPath),
            "xpath:" => Ok(IteratorType::XPath),
            "sparql:" => Ok(IteratorType::SPARQL),
            "sql:" => Ok(IteratorType::SQL),
            "csvperrow" => Ok(IteratorType::CSVRows),
            string => {
                Err(ShExMLError::ParseError(format!(
                    "{} cannot be parsed to IteratorType",
                    string
                )))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionEnum {
    ExpressionStmt(ExpressionStmt),
    MatcherExp(Matcher),
    AutoIncrementExp(AutoIncrement),
    FunctionExp(Function),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpressionReferenceIdent {
    pub source_ident:   String,
    pub iterator_ident: String,
    pub field:          Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum ExpressionStmtEnum {
    Join(Box<ExpressionStmtEnum>, Box<ExpressionStmtEnum>),

    Union(Box<ExpressionStmtEnum>, Box<ExpressionStmtEnum>),
    ConcatenateString {
        left_reference:  ExpressionReferenceIdent,
        concate_string:  String,
        right_reference: ExpressionReferenceIdent,
    },

    Basic {
        reference: ExpressionReferenceIdent,
    },
}

impl Serialize for ExpressionStmtEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExpressionStmtEnum::Join(left, right) => {
                let mut struct_serde =
                    serializer.serialize_struct("Join", 3)?;

                struct_serde.serialize_field("type", "Join")?;
                struct_serde.serialize_field("left", left)?;
                struct_serde.serialize_field("right", right)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::Union(left, right) => {
                let mut struct_serde =
                    serializer.serialize_struct("Union", 3)?;

                struct_serde.serialize_field("type", "Union")?;
                struct_serde.serialize_field("left", left)?;
                struct_serde.serialize_field("right", right)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::ConcatenateString {
                left_reference: left_path,
                concate_string,
                right_reference: right_path,
            } => {
                let mut struct_serde =
                    serializer.serialize_struct("ConcatenateString", 4)?;

                struct_serde.serialize_field("type", "ConcatenateString")?;
                struct_serde.serialize_field("left_path", left_path)?;
                struct_serde
                    .serialize_field("concate_string", concate_string)?;
                struct_serde.serialize_field("right_path", right_path)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::Basic { reference: path } => {
                let mut struct_serde =
                    serializer.serialize_struct("Basic", 2)?;

                struct_serde.serialize_field("type", "Basic")?;
                struct_serde.serialize_field("path", path)?;

                struct_serde.end()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionStmt {
    pub ident:     String,
    pub expr_enum: ExpressionStmtEnum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Matcher {
    pub ident:      String,
    pub rename_map: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoIncrement {
    pub ident:  String,
    pub start:  u32,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub end:    Option<u32>,
    pub step:   Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub ident:     String,
    pub lang_type: String,
    pub uri:       String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ShapeIdent {
    pub prefix: PrefixNameSpace,
    pub local:  String,
}

impl Display for ShapeIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.prefix, self.local)
    }
}

impl ShapeIdent {
    pub fn base() -> ShapeIdent {
        ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "".to_string(),
        }
    }
}

pub type ShapeQuads<'a> =
    (&'a Subject, &'a Predicate, &'a Object, &'a ShapeIdent);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphShapes {
    pub ident:  ShapeIdent,
    pub shapes: Vec<Shape>,
}

impl GraphShapes {
    pub fn convert_to_quads(&self) -> Vec<ShapeQuads<'_>> {
        let mut result = Vec::new();
        for (subj, pred, obj) in self
            .shapes
            .iter()
            .flat_map(|shape| shape.convert_to_triples())
        {
            result.push((subj, pred, obj, &self.ident));
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shape {
    pub ident:   ShapeIdent,
    pub subject: Subject,

    #[serde(serialize_with = "pred_obj_ser")]
    pub pred_obj_pairs: HashMap<Predicate, Object>,
}

pub type ShapeTriples<'a> = (&'a Subject, &'a Predicate, &'a Object);

impl Shape {
    pub fn convert_to_triples(&self) -> Vec<ShapeTriples<'_>> {
        let mut result = Vec::new();

        for (pred, obj) in self.pred_obj_pairs.iter() {
            result.push((&self.subject, pred, obj));
        }

        result
    }
}

fn pred_obj_ser<S>(
    pred_obj_pairs: &HashMap<Predicate, Object>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let tuples: Vec<(&Predicate, &Object)> = pred_obj_pairs.iter().collect();

    let mut seq = s.serialize_seq(Some(tuples.len()))?;

    for tup in tuples {
        seq.serialize_element(&tup)?;
    }

    seq.end()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ShapeReference {
    pub expr_ident: String,
    pub field:      Option<String>,
}

impl Display for ShapeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(field_str) = &self.field {
            write!(f, "{}.{}", self.expr_ident, field_str)
        } else {
            write!(f, "{}", self.expr_ident)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum ShapeExpression {
    #[serde(serialize_with = "shape_expr_ref_serialize")]
    Reference(ShapeReference),

    Link {
        other_shape_ident: ShapeIdent,
    },

    Static {
        value: String,
    },

    Matching {
        reference:     ShapeReference,
        matcher_ident: String,
    },

    Conditional {
        reference:        ShapeReference,
        conditional_expr: Box<ShapeExpression>,
    },

    Function {
        fun_method_ident: ShapeReference,
        params_idents:    Vec<ShapeReference>,
    },
}

impl ShapeExpression {
    pub fn extract_expr_idents(&self) -> HashSet<&str> {
        let mut result = HashSet::new();

        match self {
            ShapeExpression::Reference(reference) => {
                result.insert(reference.expr_ident.as_str());
            }
            ShapeExpression::Matching {
                reference,
                matcher_ident: _,
            } => {
                result.insert(reference.expr_ident.as_str());
            }
            ShapeExpression::Conditional {
                reference,
                conditional_expr,
            } => {
                result.insert(reference.expr_ident.as_str());
                result.extend(&conditional_expr.extract_expr_idents());
            }
            ShapeExpression::Function {
                fun_method_ident: _,
                params_idents,
            } => {
                result.extend(
                    params_idents
                        .iter()
                        .map(|shape_ref| shape_ref.expr_ident.as_str()),
                )
            }
            _ => {}
        }

        result
    }
}

fn shape_expr_ref_serialize<S>(
    shape_ref: &ShapeReference,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut struct_serde = s.serialize_struct("Reference", 1)?;

    struct_serde.serialize_field("shape_reference", shape_ref)?;
    struct_serde.end()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Object {
    pub prefix:     Option<PrefixNameSpace>,
    pub expression: ShapeExpression,
    pub language:   Option<ShapeExpression>,
    pub datatype:   Option<DataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct DataType {
    pub prefix:     Option<PrefixNameSpace>,
    pub local_expr: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Predicate {
    pub prefix: PrefixNameSpace,
    pub local:  String,
}

impl Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.prefix, self.local)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Subject {
    pub prefix:     PrefixNameSpace,
    pub expression: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PrefixNameSpace {
    #[serde(serialize_with = "ns_serialize")]
    NamedPrefix(String),
    BasePrefix,
    BNodePrefix,
}

impl FromStr for PrefixNameSpace {
    type Err = ShExMLError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(PrefixNameSpace::BasePrefix)
        } else if s == "_" {
            Ok(PrefixNameSpace::BNodePrefix)
        } else {
            Ok(PrefixNameSpace::NamedPrefix(s.to_string()))
        }
    }
}

impl Display for PrefixNameSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixNameSpace::NamedPrefix(prefix) => write!(f, "{}:", prefix),
            PrefixNameSpace::BasePrefix => write!(f, ":"),
            PrefixNameSpace::BNodePrefix => write!(f, "_:"),
        }
    }
}

fn ns_serialize<S>(ns: &String, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut serde_struct = s.serialize_struct("NamedPrefix", 1)?;

    serde_struct.serialize_field("namespace", ns)?;
    serde_struct.end()
}
