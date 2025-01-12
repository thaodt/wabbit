//! Operator handling and AST node definitions for the Wabbit compiler
//!
//! This module provides the core data structures and implementations for:
//! - Binary operators (+, -, *, /, ||, &&)
//! - Unary operators (+, -, !)
//! - Comparison operators (<, <=, >, >=, ==, !=)
//! - Expression nodes (variables, function calls, literals)
//! - Statement nodes (definitions, control flow, functions)
//! - Name handling for variables, types and functions
//!
//! The main types defined here are:
//! - `BinOpKind`, `UnaryOpKind`, `CompOpKind` - Operator enums
//! - `Expr` and `ExprKind` - Expression nodes
//! - `Stmt` and `StmtKind` - Statement nodes
//! - `Function`, `Param`, `Block` - Function-related structures
//! - `VarName`, `TypeName`, `FuncName` - Name handling types
//!
//! Each type implements relevant traits for debugging, comparison and display.

use crate::location::Span;
use std::fmt::Display;

/// Binary operators supported in Wabbit
/// with their precedence levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Or,  // Logical OR (||)
    And, // Logical AND (&&)
}

impl BinOpKind {
    pub const fn precedence(&self) -> u8 {
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::Add | BinOpKind::Sub => 4,
            Self::Mul | BinOpKind::Div => 5,
        }
    }
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Or => write!(f, "||"),
            Self::And => write!(f, "&&"),
        }
    }
}

/// Unary operators supported in Wabbit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Pos,
    Neg,
    Not, // Logical NOT (!)
}

impl UnaryOpKind {
    pub const fn precedence(&self) -> u8 {
        match self {
            Self::Pos | Self::Neg | Self::Not => 6,
        }
    }
}

impl Display for UnaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Pos => write!(f, "+"),
            Self::Not => write!(f, "!"),
        }
    }
}

/// Comparison operators supported in Wabbit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOpKind {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl CompOpKind {
    pub const fn precedence(&self) -> u8 {
        3
    }
}

impl Display for CompOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

/// Generic trait for handling named entities in Wabbit
pub trait NameModel {
    /// Creates a new named entity
    fn new(name: String) -> Self;
    /// Adds source location span information
    fn span(self, span: Span) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct NameImpl<T> {
    pub name: String,
    pub span: Span,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Display for NameImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}\'", self.name)
    }
}

impl<T> NameModel for NameImpl<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            span: Span::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

impl<T> From<&str> for NameImpl<T> {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
            span: Span::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarKind;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeKind;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncKind;

pub type VarName = NameImpl<VarKind>;
pub type TypeName = NameImpl<TypeKind>;
pub type FuncName = NameImpl<FuncKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct Comp {
    pub op: CompOpKind,
    pub right: Box<Expr>,
    pub span: Span,
}

impl Comp {
    pub fn new(op: CompOpKind, right: Expr) -> Self {
        Self {
            op,
            right: Box::new(right),
            span: Span::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Expression kinds supported in Wabbit AST
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Variable reference
    Variable(VarName),
    /// Binary operation (arithmetic/logical)
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOpKind,
        operand: Box<Expr>,
    },
    /// Comparison chain
    CompOp {
        left: Box<Expr>,
        comps: Vec<Comp>,
    },
    FuncCall {
        name: FuncName,
        args: Vec<Expr>,
    },
    /// Literal values
    Integer(i32),
    Float(f64),
    Char(char),
    Bool(bool),
}

impl ExprKind {
    pub fn precedence(&self) -> u8 {
        match self {
            ExprKind::BinOp { op, .. } => op.precedence(),
            ExprKind::UnaryOp { op, .. } => op.precedence(),
            ExprKind::CompOp { .. } => 3,
            ExprKind::Variable(_)
            | ExprKind::FuncCall { .. }
            | ExprKind::Integer(_)
            | ExprKind::Float(_)
            | ExprKind::Char(_)
            | ExprKind::Bool(_) => 255,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn variable(name: impl Into<VarName>) -> Self {
        Self {
            kind: ExprKind::Variable(name.into()),
            span: Span::default(),
        }
    }

    pub fn bin_op(op: BinOpKind, left: Expr, right: Expr) -> Self {
        Self {
            kind: ExprKind::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            span: Span::default(),
        }
    }

    pub fn unary_op(op: UnaryOpKind, operand: Expr) -> Self {
        Self {
            kind: ExprKind::UnaryOp {
                op,
                operand: Box::new(operand),
            },
            span: Span::default(),
        }
    }

    pub fn comp_op(left: Expr, comps: impl IntoIterator<Item = Comp>) -> Self {
        Self {
            kind: ExprKind::CompOp {
                left: Box::new(left),
                comps: comps.into_iter().collect(),
            },
            span: Span::default(),
        }
    }

    pub fn func_call(name: impl Into<FuncName>, args: impl IntoIterator<Item = Expr>) -> Self {
        Self {
            kind: ExprKind::FuncCall {
                name: name.into(),
                args: args.into_iter().collect(),
            },
            span: Span::default(),
        }
    }

    pub fn integer(n: i32) -> Self {
        Self {
            kind: ExprKind::Integer(n),
            span: Span::default(),
        }
    }

    pub fn float(n: f64) -> Self {
        Self {
            kind: ExprKind::Float(n),
            span: Span::default(),
        }
    }

    pub fn char(c: char) -> Self {
        Self {
            kind: ExprKind::Char(c),
            span: Span::default(),
        }
    }

    pub fn bool(b: bool) -> Self {
        Self {
            kind: ExprKind::Bool(b),
            span: Span::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Statement kinds supported in Wabbit AST
#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// Constant definition
    ConstDef {
        name: VarName,
        type_: Option<TypeName>,
        value: Expr,
    },
    /// Variable definition
    VarDef {
        name: VarName,
        type_: Option<TypeName>,
        value: Option<Expr>,
    },
    /// Assignment statement
    Assign { name: VarName, value: Expr },
    /// Print statement
    Print { expr: Expr },
    /// If-else control flow
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
    },
    /// While loop
    While { condition: Expr, block: Block },
    /// Break statement
    Break,
    /// Continue statement
    Continue,
    /// Expression statement
    Expr { expr: Expr },
    /// Function definition
    FuncDef { name: FuncName, func: Function },
    /// Return statement
    Return { expr: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn const_def(
        name: impl Into<VarName>,
        type_: impl Into<Option<TypeName>>,
        value: Expr,
    ) -> Self {
        Self {
            kind: StmtKind::ConstDef {
                name: name.into(),
                type_: type_.into(),
                value,
            },
            span: Span::default(),
        }
    }

    pub fn var_def(
        name: impl Into<VarName>,
        type_: impl Into<Option<TypeName>>,
        value: Option<Expr>,
    ) -> Self {
        Self {
            kind: StmtKind::VarDef {
                name: name.into(),
                type_: type_.into(),
                value,
            },
            span: Span::default(),
        }
    }

    pub fn assign(name: impl Into<VarName>, value: Expr) -> Self {
        Self {
            kind: StmtKind::Assign {
                name: name.into(),
                value,
            },
            span: Span::default(),
        }
    }

    pub fn print(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Print { expr },
            span: Span::default(),
        }
    }

    pub fn if_(condition: Expr, then_block: Block, else_block: Option<Block>) -> Self {
        Self {
            kind: StmtKind::If {
                condition,
                then_block,
                else_block,
            },
            span: Span::default(),
        }
    }

    pub fn while_(condition: Expr, block: Block) -> Self {
        Self {
            kind: StmtKind::While { condition, block },
            span: Span::default(),
        }
    }

    pub fn break_() -> Self {
        Self {
            kind: StmtKind::Break,
            span: Span::default(),
        }
    }

    pub fn continue_() -> Self {
        Self {
            kind: StmtKind::Continue,
            span: Span::default(),
        }
    }

    pub fn expr(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Expr { expr },
            span: Span::default(),
        }
    }

    pub fn func_def(name: impl Into<FuncName>, func: Function) -> Self {
        Self {
            kind: StmtKind::FuncDef {
                name: name.into(),
                func,
            },
            span: Span::default(),
        }
    }

    pub fn return_(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Return { expr },
            span: Span::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Function parameter definition
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: VarName,
    pub type_: TypeName,
    pub span: Span,
}

impl Param {
    pub fn new(name: impl Into<VarName>, type_: impl Into<TypeName>) -> Self {
        Self {
            name: name.into(),
            type_: type_.into(),
            span: Span::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Function definition including parameters, return type and body.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<Param>,
    pub return_type: TypeName,
    pub block: Block,
    pub span: Span,
}

impl Function {
    pub fn new(
        params: impl IntoIterator<Item = Param>,
        return_type: impl Into<TypeName>,
        block: Block,
    ) -> Self {
        Self {
            params: params.into_iter().collect(),
            return_type: return_type.into(),
            block,
            span: Span::default(),
        }
    }
    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Code block containing a sequence of statements.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self {
            stmts,
            span: Span::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
