extern crate jp;

const RADIUS: f64 = 25.0;

fn main() {
    let mut angle = 0.0;
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
                .translate(c.width() / 2.0, c.height() / 2.0)
                .rotate(angle)
                .translate(-RADIUS, -RADIUS)
                .fill([1.0, 0.0, 0.0, 1.0]),
                RADIUS * 2.0, RADIUS * 2.0);
            angle += c.dt * 5.0;
        })
        .run();
}
