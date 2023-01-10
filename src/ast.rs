#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression {
    Integer(i32),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BinaryExpression {
    pub(crate) lhs: Box<Expression>,
    pub(crate) op: BinaryOperator,
    pub(crate) rhs: Box<Expression>,
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
    Lt,
    LtEq,
    Eq,
    NotEq,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UnaryExpression {
    expr: Box<Expression>,
    op: UnaryOperator,
    // prefix: bool,
}
impl UnaryExpression {
    pub(crate) fn new(expr: Expression, op: UnaryOperator) -> Self {
        Self {
            expr: Box::new(expr),
            op,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Minus,
    // Bang,
    // Increment,
    // Decrement,
}
