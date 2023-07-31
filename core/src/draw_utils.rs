use pdf::{ContentBuilder, FontStyle};

pub fn write(
    builder: &mut ContentBuilder,
    msg: &str,
    location: (f64, f64),
    (style, font_size): &(FontStyle, f64),
) {
    builder.start_text_block();
    builder.set_font(*style, *font_size);
    builder.print_at(msg, location);
    builder.end_text_block();
}

pub fn horizontal_line(layer: &mut ContentBuilder, from: (f64, f64), length: f64) {
    layer.begin_subpath(from);
    let (x, y) = from;
    layer.line((x + length, y));
    layer.stroke_path();
}

pub fn vertical_line(layer: &mut ContentBuilder, from: (f64, f64), length: f64) {
    layer.begin_subpath(from);
    let (x, y) = from;
    layer.line((x, y + length));
    layer.stroke_path();
}

pub fn disclaimer(builder: &mut ContentBuilder) {
    let font = FontStyle::Italics;
    let font_size = 6.;
    let (page_size_x, _) = builder.page_size();
    let location = ((page_size_x / 2.) - 20., 3.);

    builder.start_text_block();
    builder.set_leading(2.);
    builder.set_font(font, font_size);
    builder.print_at("Do not use! For illustrative purposes only.", location);
    builder.next_line();
    builder.print("Any reliance you place on this document".to_owned());
    builder.next_line();
    builder.print("is strictly at your own risk.".to_owned());
    builder.next_line();
    builder.print("License: Apache-2.0".to_owned());
    builder.end_text_block();
}
