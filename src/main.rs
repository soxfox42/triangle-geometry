mod calc;

use nannou::glam::Vec3Swizzles;
use nannou::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    nannou::app(model).event(event).run();
}

#[derive(PartialEq, Eq)]
enum State {
    Entry,
    View,
}

enum Model {
    Interactive {
        state: State,
        buf: String,
        triangle: Vec<f32>,
        has_point: bool,
    },
    File {
        reader: BufReader<File>,
        triangle: Vec<f32>,
    },
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .title("Triangle Geometry")
        .size(1024, 768)
        .build()
        .unwrap();

    let mut args = env::args();
    if args.len() > 2 {
        eprintln!("Usage: {} [input_file]", args.next().unwrap());
        process::exit(1);
    }
    if args.len() == 2 {
        let path = args.skip(1).next().unwrap();
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            let mut model = Model::File {
                reader,
                triangle: Vec::new(),
            };
            step(&mut model);
            model
        } else {
            eprintln!("File not found: {}", path);
            process::exit(1);
        }
    } else {
        Model::Interactive {
            state: State::Entry,
            buf: String::new(),
            triangle: Vec::new(),
            has_point: false,
        }
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(ev), ..
    } = event
    {
        match model {
            Model::File { .. } => {
                if let KeyPressed(Key::Return) = ev {
                    step(model);
                }
            }
            Model::Interactive {
                state: State::Entry,
                ..
            } => match ev {
                KeyPressed(Key::Back) => backspace(model),
                KeyPressed(Key::Return) => submit(model),
                ReceivedCharacter(ch) => text_entry(model, ch),
                _ => (),
            },
            Model::Interactive {
                state: State::View, ..
            } => {
                if let KeyPressed(Key::Return) = ev {
                    reset(model);
                }
            }
        }
    }
}

fn step(model: &mut Model) {
    if let Model::File { reader, triangle } = model {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line.is_empty() {
            process::exit(0);
        }
        *triangle = line
            .trim()
            .split(',')
            .map(|x| x.parse().expect("Invalid triangle data"))
            .collect();
    }
}

fn backspace(model: &mut Model) {
    if let Model::Interactive { buf, has_point, .. } = model {
        let popped = buf.pop();
        if let Some('.') = popped {
            *has_point = false;
        }
    }
}

fn submit(model: &mut Model) {
    if let Model::Interactive {
        buf,
        triangle,
        state,
        has_point,
    } = model
    {
        if let Ok(val) = buf.parse() {
            triangle.push(val);
            buf.clear();
            *has_point = false;
            if triangle.len() == 6 {
                *state = State::View;
            }
        }
    }
}

fn text_entry(model: &mut Model, ch: char) {
    if let Model::Interactive { buf, has_point, .. } = model {
        if ('0'..='9').contains(&ch) {
            buf.push(ch);
        }
        if ch == '-' && buf.is_empty() {
            buf.push(ch);
        }
        if ch == '.' && !*has_point {
            buf.push(ch);
            *has_point = true;
        }
    }
}

fn reset(model: &mut Model) {
    if let Model::Interactive {
        triangle, state, ..
    } = model
    {
        triangle.clear();
        *state = State::Entry;
    }
}

const LABELS: [&str; 6] = [
    "point A's X coordinate",
    "point A's Y coordinate",
    "point B's X coordinate",
    "point B's Y coordinate",
    "point C's X coordinate",
    "point C's Y coordinate",
];

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let window_rect = app.window_rect();
    let top_bar = Rect::from_w_h(window_rect.w(), 150.0).top_left_of(window_rect);
    let top_bar_content = top_bar.pad(15.0);
    let draw_rect = Rect::from_w_h(window_rect.w(), window_rect.h() - top_bar.h())
        .below(top_bar)
        .pad(50.0);

    let draw = app.draw();

    draw.rect()
        .xy(top_bar.xy())
        .wh(top_bar.wh())
        .color(LIGHTGREY);

    match model {
        Model::Interactive {
            state: State::Entry,
            triangle,
            buf,
            ..
        } => {
            draw.text(&format!("Enter {}: {}_", LABELS[triangle.len()], buf))
                .color(BLACK)
                .font_size(24)
                .left_justify()
                .align_text_top()
                .xy(top_bar_content.xy())
                .wh(top_bar_content.wh());
        }
        Model::Interactive {
            state: State::View,
            triangle,
            ..
        }
        | Model::File { triangle, .. } => {
            let tri = geom::Tri([
                Vec2::new(triangle[0], triangle[1]),
                Vec2::new(triangle[2], triangle[3]),
                Vec2::new(triangle[4], triangle[5]),
            ]);

            let (ab, bc, ca) = calc::line_segments(tri);
            draw.text(&format!(
                "Segment Lengths: AB={:.3} BC={:.3} CA={:.3}\nArea={:.3} Perimeter={:.3}\nType: {}\nPress ENTER to continue.",
                ab,
                bc,
                ca,
                calc::area(tri),
                calc::perimeter(tri),
                calc::classify(tri),
            ))
            .color(BLACK)
            .font_size(24)
            .line_spacing(5.0)
            .left_justify()
            .align_text_top()
            .xy(top_bar_content.xy())
            .wh(top_bar_content.wh());

            let (translate, scale) = calc::view_transform(tri, draw_rect);
            let draw_transformed = draw.translate(translate).scale(scale);

            // Draw the triangle
            draw_transformed
                .tri()
                .points(tri[0], tri[1], tri[2])
                .color(Rgba::new(1.0, 0.0, 0.0, 0.5))
                .stroke(Rgb::new(1.0, 0.0, 0.0))
                .stroke_weight(5.0 / scale);

            // Draw the triangle points
            draw_transformed
                .ellipse()
                .xy(tri[0])
                .radius(8.0 / scale)
                .resolution(16.0)
                .color(BLACK);
            draw_transformed
                .ellipse()
                .xy(tri[1])
                .radius(8.0 / scale)
                .resolution(16.0)
                .color(BLACK);
            draw_transformed
                .ellipse()
                .xy(tri[2])
                .radius(8.0 / scale)
                .resolution(16.0)
                .color(BLACK);

            // Draw point labels (using manual transform to keep font size)
            draw.text(&format!("A: ({}, {})", tri[0][0], tri[0][1]))
                .xy(tri[0] * scale + translate.xy() + Vec2::new(0.0, 20.0))
                .color(BLACK)
                .font_size(18);
            draw.text(&format!("B: ({}, {})", tri[1][0], tri[1][1]))
                .xy(tri[1] * scale + translate.xy() + Vec2::new(0.0, 20.0))
                .color(BLACK)
                .font_size(18);
            draw.text(&format!("C: ({}, {})", tri[2][0], tri[2][1]))
                .xy(tri[2] * scale + translate.xy() + Vec2::new(0.0, 20.0))
                .color(BLACK)
                .font_size(18);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
