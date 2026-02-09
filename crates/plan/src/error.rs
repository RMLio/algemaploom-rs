use operator::Operator;

#[derive(thiserror::Error, Debug)]
pub enum PlanError {
    #[error("Invalid to add non-leaf operator to an empty plan")]
    EmptyPlan,

    #[error("The operator {0} is not supported")]
    OperatorNotSupported(&'static str),

    #[error(
        "The given operator cannot be added using the apply function:\n{0:?}"
    )]
    WrongApplyOperator(Operator),

    #[error("The given operator needs to be connected to a previous operator: \n{0:?}")]
    DanglingApplyOperator(Operator),

    #[error("Something else happened: {0:?}")]
    GenericError(String),
}
