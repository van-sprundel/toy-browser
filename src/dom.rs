use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use crate::{css_parser, html_parser};
use crate::css::StyleSheet;
use std::borrow::Borrow;
use std::io::Read;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    // Plain old text
    Element(ElementData),
    // <tag_name,attributes> TODO support for closed tags
    Comment(String), // <!--Comment-->
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
    pub fn get_stylesheet_from_file(&self, base_url: &str) -> Option<StyleSheet> {
        let mut res: Option<StyleSheet> = None;
        match self.node_type {
            NodeType::Element(ref e) => {
                if e.tag_name == "link" {
                    match e.attributes.get("rel") {
                        Some(ref s) => {
                            if s == &"stylesheet" {
                                let url = base_url.to_owned() + &*"\\" + e.attributes.get("href").unwrap();
                                let css = std::fs::read_to_string(url).unwrap();
                                return Some(css_parser::CssParser::new(&css).parse_stylesheet());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        for x in &self.children {
            let s = x.get_stylesheet_from_file(base_url);
            if let Some(_) = s {
                res = s
            }
        }
        res
    }
    pub fn get_stylesheet_from_url(&self, url: &str) -> Option<StyleSheet> {
        let mut res: Option<StyleSheet> = None;
        match self.node_type {
            NodeType::Element(ref e) => {
                if e.tag_name == "link" {
                    match e.attributes.get("rel") {
                        Some(ref s) => {
                            if s == &"stylesheet" {
                                let url = url.to_owned()+ e.attributes.get("href").unwrap();
                                let mut res = reqwest::blocking::get(url).unwrap();
                                let mut css = String::new();
                                res.read_to_string(&mut css).unwrap();
                                return Some(css_parser::CssParser::new(&css).parse_stylesheet());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        for x in &self.children {
            let s = x.get_stylesheet_from_url(url);
            if let Some(_) = s {
                res = s
            }
        }
        res
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
            attribute_string.push_str(&format!(" {}=\"{}\"", attr, value));
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
        println!("{}</{}>", indent, e.tag_name)
    }
}
