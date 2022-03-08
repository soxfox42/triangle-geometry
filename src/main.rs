use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

#[derive(PartialEq, Eq)]
enum State {
    Entry,
    View,
}

struct Model {
    state: State,
    buf: String,
    triangle: Vec<f32>,
    has_point: bool,
}

fn model(_app: &App) -> Model {
    Model {
        state: State::Entry,
        buf: String::new(),
        triangle: Vec::new(),
        has_point: false,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(ev), ..
    } = event
    {
        if model.state == State::View {
            if let KeyPressed(Key::Return) = ev {
                reset(model);
            }
            return;
        }
        match ev {
            KeyPressed(Key::Back) => backspace(model),
            KeyPressed(Key::Return) => submit(model),
            ReceivedCharacter(ch) => text_entry(model, ch),
            _ => (),
        }
    }
}

fn backspace(model: &mut Model) {
    let popped = model.buf.pop();
    if let Some('.') = popped {
        model.has_point = false;
    }
}

fn submit(model: &mut Model) {
    if let Ok(val) = model.buf.parse() {
        model.triangle.push(val);
        model.buf.clear();
        model.has_point = false;
        if model.triangle.len() == 6 {
            model.state = State::View;
        }
    }
}

fn text_entry(model: &mut Model, ch: char) {
    if ('0'..='9').contains(&ch) {
        model.buf.push(ch);
    }
    if ch == '-' && model.buf.is_empty() {
        model.buf.push(ch);
    }
    if ch == '.' && !model.has_point {
        model.buf.push(ch);
        model.has_point = true;
    }
}

fn reset(model: &mut Model) {
    model.triangle.clear();
    model.state = State::Entry;
}

const LABELS: [&str; 6] = [
    "first X coordinate",
    "first Y coordinate",
    "second X coordinate",
    "second Y coordinate",
    "third X coordinate",
    "third Y coordinate",
];

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let window_rect = app.window_rect();
    let top_bar = Rect::from_w_h(window_rect.w(), 150.0).top_left_of(window_rect);
    let top_bar_content = top_bar.pad(20.0);
    let draw_rect = Rect::from_w_h(window_rect.w(), window_rect.h() - top_bar.h()).below(top_bar);

    let draw = app.draw();

    draw.rect()
        .xy(top_bar.xy())
        .wh(top_bar.wh())
        .color(LIGHTGREY);
    draw.rect()
        .xy(draw_rect.xy())
        .wh(draw_rect.wh())
        .color(RED);

    match model.state {
        State::Entry => {
            draw.text(&format!(
                "Enter {}: {}",
                LABELS[model.triangle.len()],
                model.buf
            ))
            .color(BLACK)
            .font_size(24)
            .left_justify()
            .align_text_top()
            .xy(top_bar_content.xy())
            .wh(top_bar_content.wh());
        }
        State::View => {
            draw.text("Press ENTER to reset.")
                .color(BLACK)
                .font_size(24)
                .left_justify()
                .align_text_top()
                .xy(top_bar_content.xy())
                .wh(top_bar_content.wh());

            draw.tri()
                .points(
                    (model.triangle[0], model.triangle[1]),
                    (model.triangle[2], model.triangle[3]),
                    (model.triangle[4], model.triangle[5]),
                )
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
