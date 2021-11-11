use cairo::{ImageSurface, Format, Context};
use poppler::PopplerDocument;
use std::fs::File;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 200).unwrap();
    let context = Context::new(&surface);

    let poppler_document = PopplerDocument::new_from_file("/home/luis/tmp/poppler.pdf", "").unwrap();
    let poppler_page = poppler_document.get_page(0).unwrap();

    poppler_page.render(&context);

    let mut file = File::create("/home/luis/tmp/cairo.png").unwrap();
    surface.write_to_png(&mut file).unwrap();

}
