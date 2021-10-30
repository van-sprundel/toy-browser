use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

pub struct ElementData {
    pub(crate) tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> Self {
        Self {
            tag_name,
            attributes,
        }
    }
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            None => HashSet::new(),
            Some(s) => s.split(' ').collect(),
        }
    }
}

pub(crate) type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Self {
        Self {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Text(t) => write!(f, "{}", t),
            NodeType::Comment(c) => write!(f, "{}", c),
            NodeType::Element(e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut attribute_string = String::new();

        for (attr, value) in self.attributes.iter() {
            attribute_string.push_str(&format!("{}=\"{}\"", attr, value));
        }
        write!(f, "<{},{}>", self.tag_name, attribute_string)
    }
}

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(child, indent_size + 2);
    }
    if let NodeType::Element(ref e) = n.node_type {
        println!("{}</{}", indent, e.tag_name)
    }
}
