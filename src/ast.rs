#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression {
    Integer(i32),
    Binary(BinaryExpression),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BinaryExpression {
    lhs: Box<Expression>,
    op: BinaryOperator,
    rhs: Box<Expression>,
}
impl BinaryExpression {
    pub(crate) fn new(lhs: Expression, op: BinaryOperator, rhs: Expression) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Slash,
    Asterisk,
}
