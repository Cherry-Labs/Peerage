use std::thread::spawn;
use std::sync::{Arc, Mutex};
use crate::value_holder::*;

pub fn square_traversal_actor<'a>(
            node_coll: Arc<NodeColl<'a>>,
            traverser: Arc<Mutex<&'a mut Vec<StructNode<'a>>>>, 
        )