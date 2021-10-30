use browser_from_scratch::*;

fn main() {
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
    let display_commands = console.build_display_commands(&layout_tree);
    render::render_loop(&display_commands);
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
