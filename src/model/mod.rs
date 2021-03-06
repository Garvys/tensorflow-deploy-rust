use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::str;
use std::sync::Arc;

mod order;
pub use self::order::eval_order_for_nodes;

use {ops, TfdResult};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub op_name: String,
    pub inputs: Vec<OutletId>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub op: Box<ops::Op>,
}

impl Node {
    pub fn op(&self) -> &ops::Op {
        &*self.op
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct OutletId {
    pub node: usize,
    pub slot: usize,
}

impl OutletId {
    pub fn new(node: usize, slot: usize) -> OutletId {
        OutletId { node, slot }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InletId {
    pub node: usize,
    pub inlet: usize,
}

impl InletId {
    pub fn new(node: usize, inlet: usize) -> InletId {
        InletId { node, inlet }
    }
}

pub type TVec<T> = ::smallvec::SmallVec<[T; 4]>;

/// Model is Tfdeploy workhouse.
#[derive(Clone, Debug)]
pub struct RawModel {
    nodes: Vec<Node>,
    nodes_by_name: HashMap<String, usize>,
}

impl RawModel {
    pub fn new(mut nodes: Vec<Node>, nodes_by_name: HashMap<String, usize>) -> RawModel {
        let outlets: HashSet<OutletId> = nodes.iter().filter(|n| n.op_name != "Sink").map(|n| OutletId::new(n.id, 0)).collect();
        let used: HashSet<OutletId> = nodes
            .iter()
            .flat_map(|n| n.inputs.iter().cloned())
            .collect();
        for &missing in outlets.difference(&used) {
            let id = nodes.len();
            nodes.push(Node {
                id,
                name: format!("Sink-{}", id),
                op_name: "Sink".to_string(),
                inputs: vec![missing],
                op: Box::new(ops::sink::Sink::new(::analyser::TensorFact::default())),
            });
        }
        RawModel {
            nodes,
            nodes_by_name,
        }
    }

    pub fn node_by_name(&self, name: &str) -> TfdResult<&Node> {
        let id: &usize = self
            .nodes_by_name
            .get(name)
            .ok_or_else(|| format!("Node named {} not found", name))?;
        Ok(&self.nodes[*id])
    }

    pub fn node_names(&self) -> Vec<&str> {
        self.nodes.iter().map(|s| &*s.name).collect()
    }

    pub fn nodes(&self) -> &[Node] {
        &*self.nodes
    }

    pub fn guess_inputs(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.op_name == "Source")
            .collect()
    }

    pub fn guess_outputs(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.op_name == "Sink")
            .flat_map(|n|
                      n.inputs.iter().map(|i| &self.nodes[i.node]))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Model(pub Arc<RawModel>);

impl Model {
    pub fn analyser(&self, output: &str) -> TfdResult<::analyser::Analyser> {
        ::analyser::Analyser::new(&self, output)
    }
}

impl Deref for Model {
    type Target = RawModel;
    fn deref(&self) -> &RawModel {
        &*self.0
    }
}
