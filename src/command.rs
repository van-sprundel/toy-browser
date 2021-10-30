use crate::css::{Color, Value};
use crate::layout::{LayoutBox, Rectangle};

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidRectangle(Color, Rectangle),
}

pub fn build_display_commands(root: &LayoutBox) -> DisplayList {
    let mut commands = Vec::new();

    render_layout_box(&mut commands, root);
    commands
}

fn render_layout_box(commands: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(commands, layout_box);
    render_borders(commands, layout_box);

    for child in &layout_box.children {
        render_layout_box(commands, child);
    }
}

fn render_background(commands: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background-color").map(|color| {
        commands.push(DisplayCommand::SolidRectangle(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    return if let Some(v) = layout_box.styled_node.value(name) {
        if let Value::Color(ref c) = **v {
            Some(c.clone())
        } else {
            None
        }
    } else {
        None
    };
}

fn render_borders(commands: &mut DisplayList, layout_box: &LayoutBox) {
    if let Some(color) = get_color(layout_box, "border-color") {
        let d = &layout_box.dimensions;
        let border_box = d.border_box();

        // left border
        commands.push(DisplayCommand::SolidRectangle(
            color.clone(),
            Rectangle {
                x: border_box.x,
                y: border_box.y,
                width: d.border.left,
                height: border_box.height,
            },
        ));

        // right border
        commands.push(DisplayCommand::SolidRectangle(
            color.clone(),
            Rectangle {
                x: border_box.x + border_box.width - d.border.right,
                y: border_box.y,
                width: d.border.right,
                height: border_box.height,
            },
        ));

        // top border
        commands.push(DisplayCommand::SolidRectangle(
            color.clone(),
            Rectangle {
                x: border_box.x,
                y: border_box.y,
                width: border_box.width,
                height: d.border.top,
            },
        ));

        // bottom border
        commands.push(DisplayCommand::SolidRectangle(
            color.clone(),
            Rectangle {
                x: border_box.x,
                y: border_box.y + border_box.height - d.border.bottom,
                width: border_box.width,
                height: d.border.bottom,
            },
        ));
    }
}
