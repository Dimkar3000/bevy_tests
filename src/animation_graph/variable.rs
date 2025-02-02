#[derive(Debug, Clone)]
pub enum Variable {
    Bool(bool),
    Enum(String),
    Any,
}

impl Variable {
    pub fn is_any(&self) -> bool {
        matches!(self, Variable::Any)
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            (Variable::Any, _) => true,
            (_, Variable::Any) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
