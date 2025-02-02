use std::sync::Arc;

use crate::prelude::*;

#[derive(Debug)]
pub enum NodeType {
    Root(usize),                  // index of the first node
    State(Arc<str>, usize, bool), // name of state and index of the next node, the node is locking or not base on the last bool
    Switch {
        variables: Vec<usize>,     // array on indicies in the variables vector
        cases: Vec<Vec<Variable>>, // index of variable value to check against and the values that they should have in order to return true
        result: Vec<usize>,        // the index of the node we should go if we return true.
    },
    Setter(Vec<usize>, Vec<Variable>, usize), // Set the variables to the values defined
}

impl NodeType {
    pub fn is_locking(&self) -> bool {
        match self {
            NodeType::State(_, _, locking) => *locking,
            NodeType::Setter(..) => true,
            _ => false,
        }
    }
}
