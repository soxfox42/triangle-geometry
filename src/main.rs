use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    buf: String,
    triangle: Vec<f64>,
    has_point: bool,
}

fn model(_app: &App) -> Model {
    Model {
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
    }
}

fn text_entry(model: &mut Model, ch: char) {
    println!("{}", ch as u32);
    if ('0'..='9').contains(&ch) {
        model.buf.push(ch);
    }
    if ch == '.' && !model.has_point {
        model.buf.push(ch);
        model.has_point = true;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let app_rect = app.window_rect().pad(20.0);

    let draw = app.draw();

    draw.text(&format!("{}\n{:?}", model.buf, model.triangle))
        .color(BLACK)
        .font_size(24)
        .left_justify()
        .align_text_top()
        .wh(app_rect.wh());

    draw.to_frame(app, &frame).unwrap();
}
