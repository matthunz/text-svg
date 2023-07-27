use rusttype::{Font, Point, Rect, Scale};
use svg::node::element::Path;

pub struct Text {
    pub path: Path,
    pub bounding_box: Rect<f32>,
}

impl Text {
    pub fn new(path: Path, bounding_box: Rect<f32>) -> Self {
        Self { path, bounding_box }
    }

    pub fn builder() -> Builder<'static> {
        Default::default()
    }
}

pub struct Builder<'a> {
    pub fill: &'a str,
    pub size: f32,
    pub start: Point<f32>,
    pub letter_spacing: f32,
}

impl Default for Builder<'static> {
    fn default() -> Self {
        Self {
            fill: "#000",
            size: 32.,
            start: Point { x: 0., y: 0. },
            letter_spacing: 1.,
        }
    }
}

impl Builder<'_> {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn start(mut self, start: Point<f32>) -> Self {
        self.start = start;
        self
    }

    pub fn build(&self, font: &Font, text: &str) -> Text {
        let mut d = String::new();
        let mut x = self.start.x;

        let scale = Scale::uniform(self.size);
        let v_metrics = font.v_metrics(scale);
        let glyphs_height = v_metrics.ascent - v_metrics.descent;

        for glyph in font.layout(
            text,
            scale,
            Point {
                x,
                y: self.start.y + v_metrics.ascent,
            },
        ) {
            let bounding_box = glyph.unpositioned().exact_bounding_box().unwrap();
            x += bounding_box.min.x;

            glyph.build_outline(&mut crate::Builder {
                x: x,
                y: glyphs_height + bounding_box.min.y,
                d: &mut d,
            });

            x += bounding_box.width() + self.letter_spacing;
        }

        let bounding_box = Rect {
            min: self.start,
            max: Point {
                x,
                y: glyphs_height,
            },
        };
        Text::new(Path::new().set("d", d).set("fill", "#000"), bounding_box)
    }
}
