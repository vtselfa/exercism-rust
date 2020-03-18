use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    from: String,
    to: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: id.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(String::as_str)
    }
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            ..Default::default()
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs.iter() {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        for n in self.nodes.iter() {
            if n.id == id {
                return Some(n);
            }
        }
        return None;
    }
}

pub mod graph {
    pub use crate::Graph;
    pub mod graph_items {
        pub mod node {
            pub use crate::Node;
        }
        pub mod edge {
            pub use crate::Edge;
        }
    }
}
