extern crate jp;

fn main() {
    let mut pos_x = 75.0;
    let mut pos_y = 75.0;
    jp::create_window("Test", 150, 150)
        .draw(move |mut g, c| {
            g.rectangle(
                c.state()
                .fill([1.0, 1.0, 1.0, 1.0]),
                c.width(),
                c.height()
                );
            g.rectangle(
                c.state()
                .translate(pos_x, pos_y)
                .fill([1.0, 0.0, 0.0, 1.0]),
                50.0, 50.0);
            let speed = c.dt * 40.0;
            if c.is_pressed(jp::input::Button::Keyboard(jp::input::Key::Left)) {
                pos_x -= speed;
            }
            if c.is_pressed(jp::input::Button::Keyboard(jp::input::Key::Right)) {
                pos_x += speed;
            }
            if c.is_pressed(jp::input::Button::Keyboard(jp::input::Key::Up)) {
                pos_y -= speed;
            }
            if c.is_pressed(jp::input::Button::Keyboard(jp::input::Key::Down)) {
                pos_y += speed;
            }
        })
        .run();
}
