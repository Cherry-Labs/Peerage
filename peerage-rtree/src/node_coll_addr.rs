#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NodeCollAddr {
    Empty,
    Branch(usize),
    Leaf(usize)
}

impl Default for NodeCollAddr {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<(&'static str, usize)> for NodeCollAddr {
    fn from(s: (&'static str, usize)) -> Self {
        let str = s.0.to_lowercase().trim();
        let u = s.1;

        match str {
            "branch" => Self::Branch(u),
            "leaf" => Self::Leaf(u),
            _ => Self::Empty
        }
    }
}

impl Into<(&'static str, usize)> for NodeCollAddr {
    fn into(self) -> (&'static str, usize) {
        match self {
            NodeCollAddr::Empty => ("", 0),
            NodeCollAddr::Branch(u) => ("branch", u),
            NodeCollAddr::Leaf(u) => ("leaf", u),
        }
    }
}

impl NodeCollAddr {
    pub fn new_leaf(u: usize) -> Self {
        Self::Leaf(u)
    }

    pub fn new_branch(u: usize) -> Self {
        Self::Branch(u)
    }

    pub fn unwrap(&self) -> usize {
        match self {
            NodeCollAddr::Empty => 0,
            NodeCollAddr::Branch(u) => *u,
            NodeCollAddr::Leaf(u) => *usize,
        }
    }

    pub fn mutate(&mut self, u: usize) {
        match self {
            NodeCollAddr::Empty => *self = Self::Empty,
            NodeCollAddr::Branch(_) => *self = Self::Branch(u),
            NodeCollAddr::Leaf(_) => *self = Self::Leaf(u),
        }
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct PairAddress {
    branch: NodeCollAddr,
    leaf: NodeCollAddr,
}