#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Program {
    pub(crate) statements: Vec<Statement>,
}
impl Program {
    pub(crate) fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Statement {
    Expression(Expression),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Block(Vec<Statement>),
    Return(Expression),
    FunctionDefinition(FunctionDefinition),
    InitDeclaration(InitDeclaration),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IfStatement {
    pub(crate) condition: Expression,
    pub(crate) consequence: Box<Statement>,
    pub(crate) alternative: Option<Box<Statement>>,
}
impl IfStatement {
    pub(crate) fn new(
        condition: Expression,
        consequence: Statement,
        alternative: Option<Statement>,
    ) -> Self {
        Self {
            condition,
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) body: Box<Statement>,
}
impl WhileStatement {
    pub(crate) fn new(condition: Expression, body: Statement) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ForStatement {
    pub(crate) init: Option<Box<Statement>>,
    pub(crate) condition: Option<Expression>,
    pub(crate) post: Option<Box<Statement>>,
    pub(crate) body: Box<Statement>,
}
impl ForStatement {
    pub(crate) fn new(
        init: Option<Statement>,
        condition: Option<Expression>,
        post: Option<Statement>,
        body: Statement,
    ) -> Self {
        Self {
            init: init.map(Box::new),
            condition,
            post: post.map(Box::new),
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FunctionDefinition {
    pub(crate) name: String,
    pub(crate) arguments: Vec<Expression>, // Expression::LocalVariable
    pub(crate) body: Vec<Statement>,
}
impl FunctionDefinition {
    pub(crate) fn new(name: String, arguments: Vec<Expression>, body: Vec<Statement>) -> Self {
        Self {
            name,
            arguments,
            body,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InitDeclaration {
    pub(crate) name: String,
    pub(crate) offset: usize,
    pub(crate) type_: Type,
    pub(crate) init: Option<Expression>,
}
impl InitDeclaration {
    pub(crate) fn new(name: String, offset: usize, type_: Type, init: Option<Expression>) -> Self {
        Self {
            name,
            offset,
            type_,
            init,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Type {
    Primitive(TypeEnum),
    Array { type_: Box<Type>, size: i32 },
    Pointer(Box<Type>),
}
impl Type {
    pub(crate) fn size(&self) -> usize {
        match self {
            Type::Primitive(TypeEnum::Void) => 0,
            Type::Primitive(TypeEnum::Char) => 1,
            Type::Primitive(TypeEnum::Short) => 2,
            Type::Primitive(TypeEnum::Int) => 8, // FIXME: clash with 4 now.
            Type::Primitive(TypeEnum::Long) => 8,
            Type::Primitive(TypeEnum::Float) => 4,
            Type::Primitive(TypeEnum::Double) => 8,
            Type::Pointer(_) => 8,
            Type::Array { size, .. } => (size * 8) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum TypeEnum {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression {
    LocalVariable {
        name: String,
        offset: usize,
        type_: Type,
    },
    Integer(i32),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
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
    Assignment,
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
    pub(crate) expr: Box<Expression>,
    pub(crate) op: UnaryOperator,
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
    Dereference,
    Reference,
    // Bang,
    // Increment,
    // Decrement,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CallExpression {
    pub(crate) callee_name: String,
    pub(crate) arguments: Vec<Expression>,
}
impl CallExpression {
    pub(crate) fn new(callee_name: String, arguments: Vec<Expression>) -> Self {
        Self {
            callee_name,
            arguments,
        }
    }
}
