#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WabbitDataType {
    Int,
    Char,
    Bool,
    Float,
}

impl std::fmt::Display for WabbitDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Char => write!(f, "char"),
            Self::Bool => write!(f, "bool"),
            Self::Float => write!(f, "float"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WabbitValue {
    Int(i32),
    Char(char),
    Float(f64),
    Bool(bool),
    TypeHolder(WabbitDataType),
}

impl From<i32> for WabbitValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<char> for WabbitValue {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl From<bool> for WabbitValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for WabbitValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl std::fmt::Display for WabbitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bool(val) => write!(f, "{}", val),
            Self::Int(val) => write!(f, "{}", val),
            Self::Char(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::TypeHolder(val) => write!(f, "{}", val),
        }
    }
}

impl WabbitValue {
    pub fn type_of(&self) -> WabbitDataType {
        match self {
            Self::Bool(_) =>WabbitDataType::Bool,
            Self::Int(_) => WabbitDataType::Int,
            Self::Char(_) => WabbitDataType::Char,
            Self::Float(_) => WabbitDataType::Float,
            Self::TypeHolder(t) => *t,
        }
    }

    pub fn bool_compare(self, other: WabbitValue, f: impl Fn(bool, bool) -> bool) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Bool(a), WabbitValue::Bool(b)) => WabbitValue::Bool(f(a, b)),
            _ => panic!("Invalid arguments to bool_compare"),
        }
    }

    pub fn char_compare(self, other: WabbitValue, f: impl Fn(char, char) -> bool) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Char(a), WabbitValue::Char(b)) => WabbitValue::Bool(f(a, b)),
            _ => panic!("Invalid arguments to char_compare"),
        }
    }

    pub fn float_compare(self, other: WabbitValue, f: impl Fn(f64, f64) -> bool) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Float(a), WabbitValue::Float(b)) => WabbitValue::Bool(f(a, b)),
            _ => panic!("Invalid arguments to float_compare"),
        }
    }

    pub fn int_compare(self, other: WabbitValue, f: impl Fn(i32, i32) -> bool) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Int(a), WabbitValue::Int(b)) => WabbitValue::Bool(f(a, b)),
            _ => panic!("Invalid arguments to int_compare"),
        }
    }

    pub fn float_binary(self, other: WabbitValue, f: impl Fn(f64, f64) -> f64) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Float(a), WabbitValue::Float(b)) => WabbitValue::Float(f(a, b)),
            _ => panic!("Invalid arguments to float_binary"),
        }
    }

    pub fn int_binary(self, other: WabbitValue, f: impl Fn(i32, i32) -> i32) -> WabbitValue {
        match (self, other) {
            (WabbitValue::Int(a), WabbitValue::Int(b)) => WabbitValue::Int(f(a, b)),
            _ => panic!("Invalid arguments to int_binary"),
        }
    }

    pub fn float_unary(self, f: impl Fn(f64) -> f64) -> WabbitValue {
        match self {
            WabbitValue::Float(a) => WabbitValue::Float(f(a)),
            _ => panic!("Invalid arguments to float_unary"),
        }
    }

    pub fn int_unary(self, f: impl Fn(i32) -> i32) -> WabbitValue {
        match self {
            WabbitValue::Int(a) => WabbitValue::Int(f(a)),
            _ => panic!("Invalid arguments to int_unary"),
        }
    }
}

macro_rules! numeric_unary {
    ($op:ident, $loc:expr, $closure:tt) => {
        match $op {
            WabbitValue::Int(_) => Ok($op.int_unary($closure)),
            WabbitValue::Float(_) => Ok($op.float_unary($closure)),
            _ => msg!(Msg::ExpectType, $loc, "int, float"),
        }
    };
}

macro_rules! numeric_binary {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitValue::Int(_), WabbitValue::Int(_)) => Ok($op1.int_binary($op2, |a, b| a $op b)),
            (WabbitValue::Float(_), WabbitValue::Float(_)) => Ok($op1.float_binary($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float")
        }
    };
}

macro_rules! compare {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitValue::Int(_), WabbitValue::Int(_)) => Ok($op1.int_compare($op2, |a, b| a $op b)),
            (WabbitValue::Float(_), WabbitValue::Float(_)) => Ok($op1.float_compare($op2, |a, b| a $op b)),
            (WabbitValue::Char(_), WabbitValue::Char(_)) => Ok($op1.char_compare($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float, char")
        }
    };
}

macro_rules! equality {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitValue::Int(_), WabbitValue::Int(_)) => Ok($op1.int_compare($op2, |a, b| a $op b)),
            (WabbitValue::Float(_), WabbitValue::Float(_)) => Ok($op1.float_compare($op2, |a, b| a $op b)),
            (WabbitValue::Char(_), WabbitValue::Char(_)) => Ok($op1.char_compare($op2, |a, b| a $op b)),
            (WabbitValue::Bool(_), WabbitValue::Bool(_)) => Ok($op1.bool_compare($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float, char, bool")
        }
    };
}

//pub(crate) use {compare, equality, numeric_binary, numeric_unary};
