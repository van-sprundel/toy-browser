use crate::command::{DisplayCommand, DisplayCommandList, Console};
use crate::{layout, style, render, command, dom, css, html_parser, css_parser};
use iced::{Column, Container, Length, Rule, Radio, Text, Element, button, Sandbox, Settings, Align, Button, Color, Canvas, Point, Size, Scrollable, scrollable, TextInput, Row, Background};
use std::fmt::Alignment;
use crate::layout::Rectangle;
use iced::canvas::{Program, Frame, Path, Stroke, Fill, FillRule, Geometry, Cache};
use crate::command::DisplayCommand::SolidRectangle;
use iced::scrollable::{Scrollbar, Scroller};
use std::borrow::Borrow;
use std::collections::HashSet;
use iced::container::Style;
use std::iter::Peekable;
use crate::css::StyleSheet;
use std::path::PathBuf;
use std::net::{TcpStream, SocketAddr, IpAddr};
use std::io::Read;
use std::str::from_utf8;
use iced::futures::AsyncReadExt;


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

#[derive(Debug, Default, Clone)]
struct Main {
    commands: DisplayCommandList,
    url_state: iced::text_input::State,
    send_state: iced::button::State,
    url: String,
    console: Console,
}

#[derive(Debug, Clone)]
enum Message {
    PageLoaded,
    PageRefreshed(DisplayCommandList),
    UrlChanged(String),
    SendClicked,
}

impl Sandbox for Main {
    type Message = Message;

    fn new() -> Self {
        Self {
            commands: Vec::new(),
            console: Console::new(Vec::new()),
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("browser")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UrlChanged(s) => {
                self.url = s;
            }
            Message::SendClicked => {
                if let Some(c) = self.parse_url() {
                    self.commands = c;
                }
            }
            _ => {}
        }
    }

    fn view(&mut self) -> iced::Element<Message> {
        let canvas = Canvas::new(self.clone()).height(Length::Fill).width(Length::Fill);
        let url_bar = iced::TextInput::new(&mut self.url_state, "", &self.url, Message::UrlChanged).style(styling::UrlBar);
        let send_url_button = iced::Button::new(&mut self.send_state, Text::new("Send")).on_press(Message::SendClicked);
        let top_bar = Row::new().width(Length::Fill)
            .push(url_bar)
            .push(send_url_button);

        let c = Column::new()
            .push(top_bar)
            .push(canvas);

        Container::new(c)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Main {
    fn parse_url(&mut self) -> Option<DisplayCommandList> {
        let url = &self.url.to_lowercase();
        let mut path = PathBuf::from(&self.url);
        path.pop();

        // let mut url  = &self.url.to_lowercase().chars().peekable();
        // while url.peek().is_some() {
        //     self.consume_while(char::is_whitespace);
        //     if url.peek().map_or(false,|c|*c != ':') {
        //
        //     }
        // }

        let first_char = url.chars().into_iter().next().unwrap();
        let mut html = Vec::new();
        if ('a'..'z').contains(&url.chars().nth(0).unwrap()) && &url[1..3] == ":\\" {
            html = get_html_from_file(&*url);
            let ref root_node = html[0];
            dom::pretty_print(root_node, 0);

            let style_sheet = root_node.get_stylesheet_from_file(path.to_str().unwrap()).unwrap();

            let style_tree_root = style::StyledNode::new(&root_node, &style_sheet);
            style::StyledNode::pretty_print(&style_tree_root, 0);

            let mut viewport = layout::Dimensions::default();
            viewport.content.width = render::SCREEN_WIDTH as f32;
            viewport.content.height = render::SCREEN_HEIGHT as f32;

            let layout_tree = layout::LayoutBox::layout_tree(&style_tree_root, viewport);
            layout::LayoutBox::pretty_print(&layout_tree, 0);

            return Some(self.console.build_display_commands(&layout_tree));

        } else if url.starts_with("https://") || url.starts_with("http://") || first_char.is_ascii() {
            html = get_html_from_url(&*url).unwrap();
            let ref root_node = html[0];
            dom::pretty_print(root_node, 0);

            let style_sheet = root_node.get_stylesheet_from_url(url).unwrap();

            let style_tree_root = style::StyledNode::new(&root_node, &style_sheet);
            style::StyledNode::pretty_print(&style_tree_root, 0);

            let mut viewport = layout::Dimensions::default();
            viewport.content.width = render::SCREEN_WIDTH as f32;
            viewport.content.height = render::SCREEN_HEIGHT as f32;

            let layout_tree = layout::LayoutBox::layout_tree(&style_tree_root, viewport);
            layout::LayoutBox::pretty_print(&layout_tree, 0);

            return Some(self.console.build_display_commands(&layout_tree));

        } else {
            return None;
        }
       None
    }
}

impl Program<Message> for Main {
    fn draw(&self, bounds: iced::Rectangle, _: iced::canvas::Cursor) -> Vec<iced::canvas::Geometry> {
        let mut list = Vec::new();
        for command in &self.commands {
            let mut frame = Frame::new(bounds.size());
            if let SolidRectangle(ref c, ref r) = command {
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
                        color: Color {
                            r: c.r,
                            g: c.g,
                            b: c.b,
                            a: c.a,
                        },
                        rule: FillRule::NonZero,
                    },
                );
                list.push(frame.into_geometry())
            }
        }
        list
    }
}

fn get_html_from_file(p: &str) -> Vec<dom::Node> {
    let html = std::fs::read_to_string(p).unwrap();
    let nodes = html_parser::HtmlParser::new(&html).parse_nodes();
    nodes
}

fn get_html_from_url(url: &str) -> Result<Vec<dom::Node>, std::fmt::Error> {
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let nodes = html_parser::HtmlParser::new(&*body).parse_nodes();
    Ok(nodes)
}

mod styling {
    use iced::text_input::Style;
    use iced::{Color, Background};

    pub struct UrlBar;

    impl iced::text_input::StyleSheet for UrlBar {
        fn active(&self) -> Style {
            iced::text_input::Style {
                background: Background::Color(Color::WHITE),
                border_radius: 1.,
                border_width: 1.0,
                border_color: Color::BLACK,
            }
        }

        fn focused(&self) -> Style {
            iced::text_input::Style {
                background: Background::Color(Color::WHITE),
                border_radius: 1.,
                border_width: 1.4,
                border_color: Color::BLACK,
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::WHITE
        }

        fn value_color(&self) -> Color {
            iced::Color::BLACK
        }

        fn selection_color(&self) -> Color {
            Color::WHITE
        }
    }
}