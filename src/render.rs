use crate::command::{DisplayCommand, DisplayCommandList};
use crate::{layout, style, render, command, dom, css, html_parser, css_parser};
use iced::{Column, Container, Length, Rule, Radio, Text, Element, button, Sandbox, Settings, Align, Button, Color, Canvas, Point, Size, Scrollable, scrollable};
use std::fmt::Alignment;
use crate::layout::Rectangle;
use iced::canvas::{Program, Frame, Path, Stroke, Fill, FillRule};
use crate::command::DisplayCommand::SolidRectangle;
use iced::scrollable::{Scrollbar, Scroller};
use std::borrow::Borrow;

pub const SCREEN_WIDTH: usize = 960;
pub const SCREEN_HEIGHT: usize = 540;

#[derive(Copy, Clone)]
struct RenderText<'a> {
    text: &'a str,
    position: [i32; 2],
    color: [f32; 4],
}

fn transform_rectangle(rect: &layout::Rectangle) -> (f32, f32, f32, f32) {
    let w = rect.width / SCREEN_WIDTH as f32 * 2.;
    let h = rect.height / SCREEN_HEIGHT as f32 * 2.;
    let x = rect.x / SCREEN_WIDTH as f32 * 2. - 1.;
    let y = -(rect.y / SCREEN_HEIGHT as f32 * 2. - 1. + h);

    (x, y, h, w)
}

pub fn render_loop() -> iced::Result {
    Main::run(Settings::default())
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[derive(Default)]
struct Main {
    commands:DisplayCommandList,
    url: String
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PageLoaded,
    PageRefreshed,
}

impl  Sandbox for Main {
    type Message = Message;

    fn new() -> Self {
        let nodes = get_html();
        for n in nodes.iter() {
            dom::pretty_print(n, 0);
        }
        let ref root_node = nodes[0];

        let stylesheet = get_css();
        println!("{:?}", stylesheet);

        let style_tree_root = style::StyledNode::new(&root_node, &stylesheet);
        style::StyledNode::pretty_print(&style_tree_root, 0);
        // this is all correct

        let mut viewport = layout::Dimensions::default();
        viewport.content.width = render::SCREEN_WIDTH as f32;
        viewport.content.height = render::SCREEN_HEIGHT as f32;

        let layout_tree = layout::LayoutBox::layout_tree(&style_tree_root, viewport);
        layout::LayoutBox::pretty_print(&layout_tree, 0);
        let mut console = command::Console::new(Vec::new());
        let commands =  console.build_display_commands(&layout_tree);
        Self {
            commands,
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("browser")
    }

    fn update(&mut self, message:Message) {
        match message {
            _ => {}
        }
    }

    fn view(&mut self) -> iced::Element<Message> {
        let c = Column::new()
            .push(Canvas::new(self).height(Length::Fill).width(Length::Fill));

        Container::new(c).height(Length::Fill).width(Length::Fill).into()
    }
}

impl Program<Message> for Main {
    fn draw(&self, bounds: iced::Rectangle, _: iced::canvas::Cursor) -> Vec<iced::canvas::Geometry> {
        let mut list = Vec::new();
        for command in &self.commands {
            let mut frame = Frame::new(bounds.size());
            if let SolidRectangle(ref c,ref r) = command {
                frame.fill(
                    &Path::rectangle(
                        Point {
                            x: r.x,
                            y: r.y,
                        },
                        Size {
                            width: r.width,
                            height: r.height,
                        },
                    ),
                    Fill {
                        color: Color{
                            r: c.r,
                            g: c.g,
                            b: c.b,
                            a: c.a
                        },
                        rule: FillRule::NonZero
                    },
                );
                list.push(frame.into_geometry())
            }
        }
        list
    }
}


fn get_html() -> Vec<dom::Node> {
    let html = std::fs::read_to_string("./assets/example/html.html").unwrap();
    let nodes = html_parser::HtmlParser::new(&html).parse_nodes();
    nodes
}

fn get_css() -> css::StyleSheet {
    let css = std::fs::read_to_string("./assets/example/style.css").unwrap();
    css_parser::CssParser::new(&css).parse_stylesheet()
}