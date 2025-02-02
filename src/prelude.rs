pub use crate::error::*;
pub type Result<T> = std::result::Result<T, GameError>;

pub use crate::animation_graph::node_type::NodeType;
pub use crate::animation_graph::variable::Variable;
pub use crate::animation_graph::CharacterAnimationGraph;
