use crate::css::{Selector, StyleSheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

type PropertyMap<'a> = HashMap<&'a str, &'a Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap<'a>,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn new(node: &'a Node,style_sheet: &'a StyleSheet) -> Self {
        let mut style_children = Vec::new();
        let styles = match node.node_type {
            NodeType::Element(ref e) => Self::styles(e, style_sheet),
            _ => PropertyMap::new(),
        };
        for child in &node.children {
            if let NodeType::Element(_) = child.node_type {
                style_children.push(StyledNode::new(child,style_sheet));
            }
        }

        Self {
            node,
            styles,
            children: style_children,
        }
    }
    pub fn styles(element: &'a ElementData, stylesheet: &'a StyleSheet) -> PropertyMap<'a> {
        let mut styles = PropertyMap::new();

        for rule in &stylesheet.rules {
            for selector in &rule.selectors {
                if Self::selector_matches(element, &selector) {
                    for declar in &rule.declarations {
                        styles.insert(&declar.property, &declar.value);
                    }
                    break;
                }
            }
        }
        styles
    }

    pub fn value(&self, name: &str) -> Option<&&Value> {
        self.styles.get(name)
    }
    pub fn get_display(&self) -> Display {
        match self.value("display") {
            Some(Value::Other(ref v)) => match v.as_ref() {
                "block" => Display::Block,
                "inline" => Display::Inline,
                "none" => Display::None,
                "inline-block" => Display::InlineBlock,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
    pub fn num_or(&self, name: &str, default: f32) -> f32 {
        match self.value(name) {
            Some(Value::Length(n, _)) => {
                *n
            }
            _ => default,
        }
    }
    fn selector_matches(element: &ElementData, selector: &Selector) -> bool {
        for simple in &selector.simple {
            let mut selector_match = true;
            if let Some(ref t) = &simple.tag_name {
                if *t != element.tag_name {
                    continue;
                }
            }
            match element.get_id() {
                Some(i) => {
                    if let Some(ref id) = simple.id {
                        if *id != *i {
                            continue;
                        }
                    }
                }
                None => if let Some(_) = simple.id {
                    continue;
                },
            }
            let element_classes = element.get_classes();
            for class in &simple.classes {
                // boolean selector match
                selector_match &= element_classes.contains::<str>(class);
            }
            if selector_match {
                return true;
            }
        }
        false
    }
    pub fn pretty_print(node: &'a StyledNode, indent_size: usize) {
        let indent = (0..indent_size).map(|_| " ").collect::<String>();
        println!("{}{:?}", indent, node);
        for child in node.children.iter() {
            Self::pretty_print(child, indent_size + 2);
        }
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.node, self.styles)
    }
}
