use crate::css::{Color, Value};
use crate::layout::{LayoutBox, Rectangle};

pub type DisplayCommandList = Vec<DisplayCommand>;
pub struct Console {
    commands: DisplayCommandList,
}

impl Console {
    pub fn new(commands: DisplayCommandList) -> Self {
        Self { commands }
    }
    pub fn build_display_commands(&mut self, root: &LayoutBox) -> DisplayCommandList {
        self.render_layout_box(root);
        self.commands.clone()
    }
    fn render_layout_box(&mut self, layout_box: &LayoutBox) {
        self.render_background(layout_box);
        self.render_borders(layout_box);

        for child in &layout_box.children {
            self.render_layout_box(child);
        }
    }

    fn render_background(&mut self, layout_box: &LayoutBox) {
        self.get_color(layout_box, "background-color").map(|color| {
            self.commands.push(DisplayCommand::SolidRectangle(
                color,
                layout_box.dimensions.border_box(),
            ))
        });
    }

    fn get_color(&mut self, layout_box: &LayoutBox, name: &str) -> Option<Color> {
        if let Some(v) = layout_box.styled_node.value(name) {
            if let Value::Color(ref c) = **v {
                return Some(c.clone());
            } else {
                return None;
            }
        }
        return None;
    }

    fn render_borders(&mut self, layout_box: &LayoutBox) {
        if let Some(color) = self.get_color(layout_box, "border-color") {
            let d = &layout_box.dimensions;
            let border_box = d.border_box();

            // left border
            self.commands.push(DisplayCommand::SolidRectangle(
                color.clone(),
                Rectangle {
                    x: border_box.x,
                    y: border_box.y,
                    width: d.border.left,
                    height: border_box.height,
                },
            ));

            // right border
            self.commands.push(DisplayCommand::SolidRectangle(
                color.clone(),
                Rectangle {
                    x: border_box.x + border_box.width - d.border.right,
                    y: border_box.y,
                    width: d.border.right,
                    height: border_box.height,
                },
            ));

            // top border
            self.commands.push(DisplayCommand::SolidRectangle(
                color.clone(),
                Rectangle {
                    x: border_box.x,
                    y: border_box.y,
                    width: border_box.width,
                    height: d.border.top,
                },
            ));

            // bottom border
            self.commands.push(DisplayCommand::SolidRectangle(
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
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayCommand {
    SolidRectangle(Color, Rectangle),
}
