use std::{sync::Arc, time::Duration};

use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Debug)]
enum NodeType {
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
    fn is_locking(&self) -> bool {
        match self {
            NodeType::State(_, _, locking) => *locking,
            NodeType::Setter(..) => true,
            _ => false,
        }
    }
}

// starting index and count
#[derive(Debug)]
pub struct CharacterAnimation {
    pub start: usize,
    pub count: usize,
    pub flip_x: bool,
    pub animation_duration: f32, // Total time to finish the animation
}

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

impl CharacterAnimation {
    fn new(start: usize, count: usize, flip_x: bool, animation_duration: f32) -> Self {
        CharacterAnimation {
            start,
            count,
            flip_x,
            animation_duration,
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct CharacterAnimationGraph {
    animations: HashMap<Arc<str>, CharacterAnimation>,
    name_to_variable: HashMap<Arc<str>, usize>,
    variables: Vec<Variable>,

    nodes: Vec<NodeType>,
    previous_node: usize,
    current_node: usize,
    next_frame_index: usize,

    timer: Timer,

    reseted: bool,
}

impl CharacterAnimationGraph {
    pub fn get_current_animation(&self) -> Option<&CharacterAnimation> {
        let current_node = &self.nodes[self.current_node];
        match current_node {
            NodeType::State(x, _, _) => self.animations.get(x),
            _ => None,
        }
    }

    pub fn new() -> CharacterAnimationGraph {
        let animations = vec![
            (
                "standingFront".into(),
                CharacterAnimation::new(0, 6, false, 1.0),
            ),
            (
                "standingLeft".into(),
                CharacterAnimation::new(6, 6, false, 1.0),
            ),
            (
                "standingRight".into(),
                CharacterAnimation::new(6, 6, true, 1.0),
            ),
            (
                "standingBack".into(),
                CharacterAnimation::new(12, 6, false, 1.0),
            ),
            (
                "wakingFront".into(),
                CharacterAnimation::new(18, 6, false, 1.0),
            ),
            (
                "wakingLeft".into(),
                CharacterAnimation::new(24, 6, false, 1.0),
            ),
            (
                "wakingRight".into(),
                CharacterAnimation::new(24, 6, true, 1.0),
            ),
            (
                "wakingBack".into(),
                CharacterAnimation::new(30, 6, false, 1.0),
            ),
            (
                "attackingFront".into(),
                CharacterAnimation::new(36, 4, false, 0.4),
            ),
            (
                "attackingLeft".into(),
                CharacterAnimation::new(42, 4, false, 0.4),
            ),
            (
                "attackingRight".into(),
                CharacterAnimation::new(42, 4, true, 0.4),
            ),
            (
                "attackingBack".into(),
                CharacterAnimation::new(48, 4, true, 0.4),
            ),
        ]
        .into_iter()
        .collect();
        let variables = vec![
            Variable::Enum("none".into()),
            Variable::Enum("down".into()),
            Variable::Bool(false),
            Variable::Bool(false),
        ];

        let name_to_variable = vec![
            ("directionX".into(), 0),
            ("directionY".into(), 1),
            ("walking".into(), 2),
            ("attacking".into(), 3),
        ]
        .into_iter()
        .collect();

        let nodes = vec![
            NodeType::Root(1),
            NodeType::Switch {
                variables: vec![0, 1, 2, 3],
                cases: vec![
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("left".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("left".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("right".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("right".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(false),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("left".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("left".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("right".into()),
                        Variable::Enum("up".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    vec![
                        Variable::Enum("right".into()),
                        Variable::Enum("down".into()),
                        Variable::Bool(true),
                        Variable::Bool(false),
                    ],
                    // Attacking
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("down".into()),
                        Variable::Any,
                        Variable::Bool(true),
                    ],
                    vec![
                        Variable::Enum("none".into()),
                        Variable::Enum("up".into()),
                        Variable::Any,
                        Variable::Bool(true),
                    ],
                    vec![
                        Variable::Enum("left".into()),
                        Variable::Any,
                        Variable::Any,
                        Variable::Bool(true),
                    ],
                    vec![
                        Variable::Enum("right".into()),
                        Variable::Any,
                        Variable::Any,
                        Variable::Bool(true),
                    ],
                ],
                result: vec![2, 5, 3, 3, 4, 4, 6, 9, 7, 7, 8, 8, 10, 13, 11, 12],
            },
            NodeType::State("standingFront".into(), 1, false),
            NodeType::State("standingLeft".into(), 1, false),
            NodeType::State("standingRight".into(), 1, false),
            NodeType::State("standingBack".into(), 1, false),
            NodeType::State("wakingFront".into(), 1, false),
            NodeType::State("wakingLeft".into(), 1, false),
            NodeType::State("wakingRight".into(), 1, false),
            NodeType::State("wakingBack".into(), 1, false),
            NodeType::State("attackingFront".into(), 14, true),
            NodeType::State("attackingLeft".into(), 14, true),
            NodeType::State("attackingRight".into(), 14, true),
            NodeType::State("attackingBack".into(), 14, true),
            NodeType::Setter(vec![3], vec![Variable::Bool(false)], 1), // Set attacking false and re-eval
        ];

        CharacterAnimationGraph {
            animations,
            variables,
            name_to_variable,
            nodes,
            current_node: 0,
            previous_node: 1,
            next_frame_index: 0,
            timer: Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Once),
            reseted: false,
        }
    }

    pub fn set_variable(&mut self, name: impl AsRef<str>, value: Variable) {
        if value.is_any() {
            warn!("cannot set a variable to ANY. Skipping...");
            return;
        }
        let name = name.as_ref();

        if let Some(v) = self.name_to_variable.get(name) {
            if self.variables[*v] != value {
                self.variables[*v] = value;
                self.reseted = true;
            }
        }
    }

    fn eval(&mut self) -> Option<usize> {
        loop {
            let current_node = &self.nodes[self.current_node];
            match current_node {
                NodeType::Root(x) => {
                    self.previous_node = self.current_node;
                    self.current_node = *x;
                    continue;
                }
                NodeType::State(x, y, _) => {
                    let state = self.animations.get(x).unwrap();

                    // If the current frame is 0 we reset it to were it should be.
                    if self.next_frame_index < state.start {
                        self.next_frame_index = state.start;
                    }

                    let current_frame = self.next_frame_index;
                    let start_frame = state.start;
                    let count = state.count;

                    self.next_frame_index = current_frame + 1;

                    // Animation finished moving to next node
                    if self.next_frame_index > start_frame + count - 1 {
                        self.next_frame_index = 0;
                        self.current_node = *y;
                    }
                    return Some(current_frame);
                }
                NodeType::Switch {
                    variables,
                    cases,
                    result,
                } => {
                    let state = variables
                        .iter()
                        .map(|v| &self.variables[*v])
                        .collect::<Vec<_>>();
                    let evaluation = cases
                        .iter()
                        .position(|case| case.iter().zip(state.iter()).all(|(x, y)| &x == y));

                    if let Some(position) = evaluation {
                        let position = result[position];
                        self.current_node = position;
                    } else {
                        panic!("graph switch evaluation failed.\nState: {:?}", state);
                    }
                }
                NodeType::Setter(variables, values, next) => {
                    for (index, variable_index) in variables.iter().enumerate() {
                        if let Some(variable) = self.variables.get_mut(*variable_index) {
                            if let Some(new_val) = values.get(index) {
                                if new_val.is_any() {
                                    warn!("cannot set a variable to ANY. Skipping...");
                                    break;
                                }
                                // TODO: Check for any, we should not allow it.
                                *variable = new_val.clone();
                            }
                        }
                    }
                    self.current_node = *next;
                }
            }
        }
    }

    pub fn get_next_index(&mut self, delta: f32) -> Option<usize> {
        let current_node = &self.nodes[self.current_node];

        if self.reseted && !current_node.is_locking() {
            self.current_node = 0; //Root Node
            self.next_frame_index = 0;
            self.reseted = false;
            return self.eval();
        }
        self.timer.tick(Duration::from_secs_f32(delta));
        if !self.timer.finished() {
            return None;
        }

        let result = self.eval();

        if let Some(animation) = self.get_current_animation() {
            // Calculate the time we overshoot.
            let over = self.timer.elapsed() - self.timer.duration();
            self.timer = Timer::new(
                Duration::from_secs_f32(animation.animation_duration / animation.count as f32),
                TimerMode::Once,
            );

            // Add the overshoot time to the next timer. This way the timings are going to be more accurate.
            self.timer.tick(over);
        }

        result
    }
}
