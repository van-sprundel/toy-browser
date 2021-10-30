use std::fmt;

use crate::css::{Unit, Value};
use crate::style::{Display, StyledNode};
use std::fmt::Formatter;

pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    box_type: BoxType,
    pub styled_node: &'a StyledNode<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType, styled_node: &'a StyledNode) -> Self {
        Self {
            dimensions: Default::default(),
            box_type,
            styled_node,
            children: Vec::new(),
        }
    }
    fn layout(&mut self, b_box: Dimensions) {
        match self.box_type {
            BoxType::Block => self.layout_block(b_box),
            BoxType::Inline => self.layout_block(b_box),
            BoxType::InlineBlock => self.layout_inline_block(b_box),
            BoxType::Anonymous => {}
        }
    }

    fn layout_block(&mut self, b_box: Dimensions) {
        self.calculate_width(b_box);
        self.calculate_position(b_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_width(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        let width = Self::absolute_num(s, b_box, "width").unwrap_or(0.);
        let margin_l = s.value("margin-left");
        let margin_r = s.value("margin-right");

        let margin_l_num = if let Some(m) = margin_l {
            if let Value::Other(ref s) = **m {
                s.parse().unwrap_or(0.)
            } else {
                0.
            }
        } else {
            0.
        };

        let margin_r_num = if let Some(m) = margin_r {
            if let Value::Other(ref s) = **m {
                s.parse().unwrap_or(0.)
            } else {
                0.
            }
        } else {
            0.
        };

        d.border.left = s.num_or("border-left-width", 0.);
        d.border.right = s.num_or("border-right-width", 0.);
        d.padding.left = s.num_or("padding-left", 0.);
        d.padding.right = s.num_or("padding-right", 0.);

        let total = width
            + margin_l_num
            + margin_r_num
            + d.border.left
            + d.border.right
            + d.padding.left
            + d.padding.right;

        let underflow = b_box.content.width - total;

        match (width, margin_l, margin_r) {
            (w, None, Some(_)) if w != 0. => {
                d.margin.left = underflow;
                d.margin.right = margin_r_num;
                d.content.width = w;
            }
            (w, Some(_), None) if w != 0. => {
                d.margin.right = underflow;
                d.margin.left = margin_l_num;
                d.content.width = w;
            }
            (w, None, None) if w != 0. => {
                d.margin.right = underflow / 2.;
                d.margin.left = underflow / 2.;
                d.content.width = w;
            }
            (w, _, _) => {
                if w == 0. {
                    if underflow >= 0. {
                        d.content.width = underflow;
                        d.margin.right = margin_r_num;
                    } else {
                        d.content.width = width;
                        d.margin.right = margin_r_num + underflow;
                    }
                } else {
                    d.margin.right = margin_r_num + underflow;
                    d.margin.left = margin_l_num;
                    d.content.width = width;
                }
            }
        }
    }

    fn calculate_position(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.margin.top = s.num_or("margin-top", 0.);
        d.margin.bottom = s.num_or("margin-bottom", 0.);
        d.border.top = s.num_or("border-top-width", 0.);
        d.border.bottom = s.num_or("border-bottom-width", 0.);
        d.padding.top = s.num_or("padding-top", 0.);
        d.padding.bottom = s.num_or("padding-bottom", 0.);

        d.content.x = b_box.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y =
            b_box.content.height + b_box.content.y + d.margin.top + d.border.top + d.padding.top;
    }

    fn calculate_height(&mut self) {
        self.styled_node.value("height").map_or((), |h| {
            if let Value::Length(n, _) = **h {
                self.dimensions.content.height = n;
            }
        })
    }

    fn layout_children(&mut self) {
        let d = &mut self.dimensions;
        let mut max_child_height = 0.;

        let mut prev_box_type = BoxType::Block;

        for child in &mut self.children {
            if let BoxType::InlineBlock = prev_box_type {
                if let BoxType::Block = child.box_type {
                    d.content.height += max_child_height;
                    d.current.x = 0.;
                }
            }

            child.layout(*d);
            let new_height = child.dimensions.margin_box().height;

            if new_height > max_child_height {
                max_child_height = new_height;
            }

            match child.box_type {
                BoxType::Block => d.content.height += child.dimensions.margin_box().height,
                BoxType::InlineBlock => {
                    d.current.x += child.dimensions.margin_box().width;

                    if d.current.x > d.content.width {
                        d.content.height += max_child_height;
                        d.current.x = 0.;
                        child.layout(*d);
                        d.current.x += child.dimensions.margin_box().width;
                    }
                }
                _ => {}
            }

            prev_box_type = child.box_type.clone();
        }
    }

    fn layout_inline_block(&mut self, b_box: Dimensions) {
        self.calculate_inline_width(b_box);
        self.calculate_inline_position(b_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_inline_width(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.content.width = Self::absolute_num(s, b_box, "width").unwrap_or(0.);
        d.margin.left = s.num_or("margin-left", 0.0);
        d.margin.right = s.num_or("margin-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left", 0.0);
        d.border.right = s.num_or("border-right", 0.0);
    }

    fn calculate_inline_position(&mut self, b_box: Dimensions) {
        let s = self.styled_node;
        let d = &mut self.dimensions;

        d.margin.top = s.num_or("margin-top", 0.0);
        d.margin.bottom = s.num_or("margin-bottom", 0.0);
        d.padding.top = s.num_or("padding-top", 0.0);
        d.padding.bottom = s.num_or("padding-bottom", 0.0);
        d.border.top = s.num_or("border-top", 0.0);
        d.border.bottom = s.num_or("border-bottom", 0.0);

        d.content.x =
            b_box.content.x + b_box.current.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y =
            b_box.content.height + b_box.content.y + d.margin.top + d.border.top + d.padding.top;
    }

    fn absolute_num(s_node: &StyledNode, b_box: Dimensions, prop: &str) -> Option<f32> {
        if let Some(ref v) = s_node.value(prop) {
            if let Value::Length(l, ref u) = ***v {
                return match *u {
                    Unit::Px => Some(l),
                    Unit::Pct => Some(l * b_box.content.width / 100.),
                    _ => todo!(),
                };
            }
        }
        None
    }
    pub fn layout_tree(
        root: &'a StyledNode<'a>,
        mut containing_block: Dimensions,
    ) -> LayoutBox<'a> {
        containing_block.content.height = 0.;
        let mut root_box = Self::build_layout_tree(root);
        root_box.layout(containing_block);
        root_box
    }
    fn build_layout_tree(node: &'a StyledNode) -> LayoutBox<'a> {
        let mut layout_node = LayoutBox::new(
            match node.get_display() {
                Display::Block => BoxType::Block,
                Display::InlineBlock => BoxType::InlineBlock,
                Display::Inline => BoxType::Inline,
                Display::None => BoxType::Anonymous,
            },
            node,
        );

        for child in &node.children {
            match child.get_display() {
                Display::Block => layout_node.children.push(Self::build_layout_tree(child)),
                Display::Inline => layout_node.children.push(Self::build_layout_tree(child)),
                Display::InlineBlock => layout_node.children.push(Self::build_layout_tree(child)),
                Display::None => {}
            }
        }
        layout_node
    }
    pub fn pretty_print(n: &LayoutBox, level: usize) {
        println!("{}{:?}\n", level, n);

        for child in n.children.iter() {
            Self::pretty_print(&child, level + 1);
        }
    }
}

impl<'a> fmt::Debug for LayoutBox<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "type:\n {:?}\n{:?}\n", self.box_type, self.dimensions)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Dimensions {
    pub content: Rectangle,
    pub border: EdgeSizes,
    padding: EdgeSizes,
    margin: EdgeSizes,
    current: Rectangle,
}

impl Dimensions {
    fn padding_box(&self) -> Rectangle {
        self.content.expanded(self.padding)
    }
    pub fn border_box(&self) -> Rectangle {
        self.padding_box().expanded(self.border)
    }
    fn margin_box(&self) -> Rectangle {
        self.border_box().expanded(self.margin)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    fn expanded(&self, e: EdgeSizes) -> Self {
        Self {
            x: self.x - e.left,
            y: self.y - e.top,
            width: self.width + e.left + e.right,
            height: self.height + e.top + e.bottom,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Clone)]
pub enum BoxType {
    Block,
    Inline,
    InlineBlock,
    Anonymous,
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {}, w: {}, h: {}",
            self.x, self.y, self.width, self.height
        )
    }
}
impl fmt::Debug for EdgeSizes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "l: {} r: {} top: {} bot: {}",
            self.left, self.right, self.top, self.bottom
        )
    }
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "content:\n  {:?}\npadding:\n  {:?}\nborder:\n  {:?}\nmargin:\n  {:?}",
            self.content, self.padding, self.border, self.margin
        )
    }
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_type = match *self {
            BoxType::Block => "block",
            BoxType::Inline => "inline",
            BoxType::InlineBlock => "inline-block",
            BoxType::Anonymous => "anonymous",
        };

        write!(f, "{}", display_type)
    }
}
