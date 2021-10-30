use gfx;
use gfx_text;
use gfx_window_glutin;
use glutin;

use crate::command::DisplayCommand;
use crate::layout;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::*;
use glutin::{Event, WindowEvent};

pub type DepthFormat = gfx::format::DepthStencil;
pub type ColorFormat = gfx::format::Rgba8;

pub const SCREEN_WIDTH: usize = 960;
pub const SCREEN_HEIGHT: usize = 540;
const CLEAR_COLOR: [f32; 4] = [1., 1., 1., 1.];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

#[derive(Copy, Clone)]
struct RenderText<'a> {
    text: &'a str,
    position: [i32; 2],
    color: [f32; 4],
}

fn render_texts(_command_list: &[DisplayCommand]) -> Vec<RenderText> {
    Vec::new()
}

fn render_commands(command_list: &[DisplayCommand]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    let mut rect_num: u16 = 0;

    for command in command_list {
        if let DisplayCommand::SolidRectangle(ref color, ref rect) = command {
            let c = [color.r, color.g, color.b];
            let mut v = render_rectangle(&c, rect);
            vertices.append(&mut v);

            let index_base = rect_num * 4;
            index_data.append(&mut vec![
                index_base,
                index_base + 1,
                index_base + 2,
                index_base + 2,
                index_base + 3,
                index_base,
            ]);
            rect_num += 1;
        }
    }
    return (vertices, index_data);
}

fn render_rectangle(c: &[f32; 3], rect: &layout::Rectangle) -> Vec<Vertex> {
    let (x, y, h, w) = transform_rectangle(rect);
    let vertices = vec![
        Vertex {
            pos: [x + w, y],
            color: *c,
        },
        Vertex {
            pos: [x, y],
            color: *c,
        },
        Vertex {
            pos: [x, y + h],
            color: *c,
        },
        Vertex {
            pos: [x + w, y + h],
            color: *c,
        },
    ];
    vertices
}

fn transform_rectangle(rect: &layout::Rectangle) -> (f32, f32, f32, f32) {
    let w = rect.width / SCREEN_WIDTH as f32 * 2.;
    let h = rect.height / SCREEN_HEIGHT as f32 * 2.;
    let x = rect.x / SCREEN_WIDTH as f32 * 2. - 1.;
    let y = -(rect.y / SCREEN_HEIGHT as f32 * 2. - 1. + h);

    (x, y, h, w)
}

pub fn render_loop(command_list: &[DisplayCommand]) {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new().with_title("browser".to_owned());
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let (window, mut device, mut factory, rtv, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop)
            .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("../assets/shaders/solid.glslv"),
            include_bytes!("../assets/shaders/solid.glslf"),
            pipe::new(),
        )
        .unwrap();

    let (vertices, index_data) = render_commands(command_list);
    let texts = render_texts(command_list);

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&vertices, &index_data[..]);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: rtv,
    };

    let mut test_renderer = gfx_text::new(factory).build().unwrap();

    let mut running = true;
    'main: loop {
        if !running {
            break 'main;
        }
        events_loop.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => running = false,
                WindowEvent::KeyboardInput { .. } => running = false,
                _ => {}
            },
            _ => {}
        });
        for text in &texts {
            test_renderer.add(text.text, text.position, text.color);
        }

        encoder.clear(&data.out, CLEAR_COLOR);

        encoder.draw(&slice, &pso, &data);
        test_renderer.draw(&mut encoder, &data.out).unwrap();

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
