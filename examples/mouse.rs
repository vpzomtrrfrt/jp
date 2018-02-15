extern crate jp;

fn main() {
    jp::create_window("Test", 150, 150)
        .draw(|mut g, c| {
            g.rectangle(
                c.state()
                .fill([1.0, 1.0, 1.0, 1.0]),
                c.width(),
                c.height()
                );
            g.rectangle(
                c.state()
                .translate(c.mouse_x, c.mouse_y)
                .fill([1.0, 0.0, 0.0, 1.0]),
                50.0, 50.0);
        })
        .run();
}
