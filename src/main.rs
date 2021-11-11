use cairo::{ImageSurface, Format, Context};
use std::fs::File;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 200).unwrap();
    let context = Context::new(&surface).unwrap();

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    context.set_source_rgb(0.0, 0.0, 0.0);
    for y in 0..100 {
        let yf = y as f64 / 100.0 * 600.0;
        for x in 0..100 {
            let xf = x as f64 / 100.0 * 200.0;
            context.line_to(xf, yf);
        }
    }
    context.stroke().unwrap();

    let mut file = File::create("/home/luis/tmp/cairo.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
