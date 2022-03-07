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
    triangle: Vec<f64>,
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

    let app_rect = app.window_rect().pad(20.0);

    let draw = app.draw();

    match model.state {
        State::Entry => {
            draw.text(&format!(
                "Enter {}: {}\n{:?}",
                LABELS[model.triangle.len()],
                model.buf,
                model.triangle
            ))
            .color(BLACK)
            .font_size(24)
            .left_justify()
            .align_text_top()
            .wh(app_rect.wh());
        }
        State::View => {
            draw.text("Displaying triangle\nPress enter to reset").color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
