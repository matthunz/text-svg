use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::{Font, Point};
use std::{fs::File, io::Read};
use svg::{node::element::Rectangle, Document};
use text_svg::Text;

fn main() {
    let handle = SystemSource::new()
        .select_best_match(&[FamilyName::Serif], &Properties::new())
        .unwrap();

    let font = match handle {
        Handle::Path { path, font_index } => {
            let mut file = File::open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            Font::try_from_vec_and_index(buf, font_index).unwrap()
        }
        Handle::Memory { bytes, font_index } => {
            Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
        }
    };

    let x = 10.;
    let y = 20.;

    let text = Text::builder()
        .size(50.0)
        .start(Point { x, y })
        .build(&font, "text-svg");

    let document = Document::new()
        .set("width", text.bounding_box.max.x + x)
        .set("height", text.bounding_box.max.y + y)
        .add(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0.)
                .set("y", 0.)
                .set("width", text.bounding_box.max.x + x)
                .set("height", text.bounding_box.max.y + y),
        )
        .add(text.path);

    svg::save("image.svg", &document).unwrap();
}
