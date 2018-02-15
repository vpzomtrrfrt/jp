extern crate jp;

fn main() {
    jp::create_window("Test", 150, 150)
        .draw(|mut g, c| {
            g.rectangle(
                c.state()
                .translate(c.width() / 2.0, c.height() / 2.0)
                .fill([1.0, 0.0, 0.0, 1.0]),
                50.0, 50.0);
        })
        .run();
}
