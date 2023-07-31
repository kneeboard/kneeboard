use crate::{
    calc::{calc_aircraft, Degree, Velocity},
    definition::Leg as JSonLeg,
    definition::{Detail, FontType},
    draw_utils::{disclaimer, horizontal_line, vertical_line, write},
};

use pdf::{init_page, ContentBuilder, FontStyle, PDFPageBuilder};

const MARGIN_SIDE: f64 = 2.5;
const FONT_SIZE: f64 = 10.;
const FONT_NOTES_SIZE: f64 = 9.;
const FONT_HEADER_SIZE: f64 = 7.;

pub fn create_plog(legs: &[Leg], notes: &[FontType], detail: &Detail, page: &mut PDFPageBuilder) {
    let calc_legs = calc_legs(legs);

    let mut layer = page.content_builder();
    init_page(&mut layer);
    disclaimer(&mut layer);

    layer.save_graphics_state();
    layer.line_width(0.25);

    let x = MARGIN_SIDE;
    let mut y = 20.;

    let name_height = 4.;

    let columns = [
        (25., Some("Safe")),
        (7., Some("Plan")),
        (7., Some("Spd")),
        (7., Some("Track")),
        (7., Some("Dist")),
        (7., Some("Wind")),
        (15., Some("G/S")),
        (7., Some("HD(T)")),
        (8., Some("HD(M)")),
        (0.5, None),
        (9., Some("Time")),
        (7., Some("S/C")),
        (14., Some("ETA")),
        (14., Some("ATA")),
    ];

    let (page_width, _) = layer.page_size();

    let line_inc = |y: f64| y + 2. + name_height * 2.;
    for (leg, leg_calc) in calc_legs {
        layer.begin_subpath((x, y));
        layer.line((page_width - (2. * MARGIN_SIDE), y));
        layer.stroke_path();

        let (from, to) = &leg.name;

        let y_top_text = y + name_height;
        let y_bottom_text = 1. + y + name_height * 2.;

        let y_middle_text = (y_top_text + y_bottom_text) / 2.;

        layer.start_text_block();
        layer.set_font(FontStyle::Normal, FONT_SIZE);
        layer.set_leading(4.5);
        layer.print_at(from, (x, y_top_text));
        layer.next_line();
        layer.print(to.to_owned());
        layer.end_text_block();

        let wind = format!(
            "{}@{}",
            leg.wind_direction.as_heading().as_str(),
            leg.wind_speed
        );

        let heading = leg_calc.heading.as_heading();
        let heading_magnetic = leg_calc.heading_magnetic.as_heading();
        let values = [
            (&leg.safe, 0., FontStyle::Normal),
            (&leg.planned, 0., FontStyle::Normal),
            (&leg.speed.as_string(), 0., FontStyle::Normal),
            (&leg.course.as_heading(), 0., FontStyle::Normal),
            (&leg.distance.as_string(), 0., FontStyle::Normal),
            (&wind, 0., FontStyle::Normal),
            (&leg_calc.ground_speed.as_string(), 0., FontStyle::Normal),
            (&heading, 0.5, FontStyle::Normal),
            (&heading_magnetic, 1., FontStyle::Bold),
            (&"".to_owned(), 0., FontStyle::Normal),
            (&leg_calc.time.as_string(), 0.5, FontStyle::Bold),
        ];

        let mut x = 0.;
        for ((value, adjust, font), (x_offset, heading)) in values.iter().zip(columns.iter()) {
            let calc_font = (*font, FONT_SIZE);

            x += x_offset;

            if heading.is_some() {
                write(&mut layer, value, (x + adjust, y_middle_text), &calc_font);
            }
        }

        y = line_inc(y);
    }

    {
        layer.start_text_block();
        layer.set_font(FontStyle::Normal, FONT_SIZE);
        layer.set_leading(4.);
        layer.print_at("LOST", (x, 6.));
        layer.next_line();
        layer.print("121.5".to_owned());
        layer.next_line();
        layer.print("0030".to_owned());
        layer.end_text_block()
    }

    {
        layer.start_text_block();
        layer.set_font(FontStyle::Normal, FONT_SIZE);
        layer.set_leading(4.);
        for (n, id) in vec![&detail.tail, &detail.call_sign, &detail.pic]
            .into_iter()
            .flatten()
            .enumerate()
        {
            if n == 0 {
                layer.print_at(id, (page_width - 25., 6.));
            } else {
                layer.next_line();
                layer.print(id.to_owned());
            }
        }
        layer.end_text_block();
    }

    let mut divider_x = 0.;
    for (x_offset, heading) in columns {
        divider_x += x_offset;
        if let Some(head) = heading {
            layer.start_text_block();
            layer.set_font(FontStyle::Bold, FONT_HEADER_SIZE);
            layer.print_at(head, (divider_x, 19.));
            layer.end_text_block()
        }
        vertical_line(&mut layer, (divider_x - 0.5, 20.), -(20. - y));
    }

    horizontal_line(&mut layer, (x, y), page_width - (2.0 * MARGIN_SIDE));

    {
        y = line_inc(y);
        horizontal_line(&mut layer, (x, y), page_width - (2. * MARGIN_SIDE));

        let y_txt = y - 4.;
        let mut x_txt = x;

        let font = (FontStyle::Normal, FONT_SIZE);

        write(&mut layer, "Oil:", (x_txt, y_txt), &font);
        x_txt += 15.;
        write(&mut layer, "Fuel:", (x_txt, y_txt), &font);
        x_txt += 25.;
        write(&mut layer, "B/Off:", (x_txt, y_txt), &font);
        x_txt += 25.;
        write(&mut layer, "T/Off:", (x_txt, y_txt), &font);
        x_txt += 25.;
        write(&mut layer, "Lnd:", (x_txt, y_txt), &font);
        x_txt += 25.;
        write(&mut layer, "B/On:", (x_txt, y_txt), &font);
    }

    let fuel_y = create_fuel((page_width - 52.5, y), &mut layer);
    write_notes((page_width - 52.5, fuel_y + 5.), notes, &mut layer);
}

fn write_notes(start: (f64, f64), notes: &[FontType], layer: &mut ContentBuilder) {
    let (x, y) = start;

    layer.start_text_block();
    layer.set_leading(3.5);

    let empty = "".to_owned();
    for (n, note) in notes.iter().enumerate() {
        let current = match note {
            FontType::Normal(ref txt) => Some((FontStyle::Normal, txt)),
            FontType::Bold(ref txt) => Some((FontStyle::Bold, txt)),
            FontType::Italics(ref txt) => Some((FontStyle::Italics, txt)),
            FontType::Blank => None,
        };

        let (font, txt) = if let Some((font, txt)) = current {
            (font, txt)
        } else {
            (FontStyle::Normal, &empty)
        };

        layer.set_font(font, FONT_NOTES_SIZE);
        if n == 0 {
            layer.print_at(txt, (x, y));
        } else {
            layer.next_line();
            layer.print(txt.to_owned())
        }
    }
    layer.end_text_block();
}

fn create_fuel(start: (f64, f64), layer: &mut ContentBuilder) -> f64 {
    let width = 50.;

    let name_height = 4.;
    let (x, mut y) = start;
    for _ in 0..4 {
        let line_start = (x, y);
        horizontal_line(layer, line_start, width);

        layer.start_text_block();
        layer.set_font(FontStyle::Bold, FONT_SIZE);
        layer.print_at("L    R", (x + 3., y + name_height + 2.));
        layer.end_text_block();

        y += name_height + 5.;
    }

    {
        let line_start = (x, y);
        horizontal_line(layer, line_start, 50.);
    }

    {
        let (box_x, box_y) = start;

        {
            let line_start = (box_x, box_y);
            vertical_line(layer, line_start, y - box_y);
        }

        {
            let line_start = (box_x + 15., box_y);
            vertical_line(layer, line_start, y - box_y);
        }

        let gap = (width - 15.) / 3.;

        {
            let line_start = (box_x + 15. + gap, box_y);
            vertical_line(layer, line_start, y - box_y);
        }

        {
            let line_start = (box_x + 15. + 2. * gap, box_y);
            vertical_line(layer, line_start, y - box_y);
        }

        {
            let line_start = (box_x + width, box_y);
            vertical_line(layer, line_start, y - box_y);
        }
    }

    y
}

trait FloatToString {
    fn as_string(&self) -> String;
}

impl FloatToString for f64 {
    fn as_string(&self) -> String {
        (self.round() as i32).to_string()
    }
}

pub fn calc_legs(legs: &[Leg]) -> Vec<(&Leg, LegCalc)> {
    let mut result = vec![];

    let mut total = 0.;
    for leg in legs {
        let wind = Velocity {
            speed: leg.wind_speed,
            bearing: leg.wind_direction.reciprocal(),
        };

        let destination_bearing = leg.course;
        let variation = leg.variation;
        let heading = calc_aircraft(leg.speed, destination_bearing, variation, &wind);

        let ground_speed = heading.speed_overground;
        let heading_magnetic = heading.heading_magnetic;
        let heading = heading.heading;

        let time = 60.0 * leg.distance / ground_speed;

        total += time;
        let leg_calc = LegCalc {
            ground_speed,
            heading,
            heading_magnetic,
            time,
            total,
        };

        result.push((leg, leg_calc));
    }

    result
}

pub struct LegCalc {
    pub ground_speed: f64,
    pub heading: Degree,
    pub heading_magnetic: Degree,
    pub time: f64,
    pub total: f64,
}

pub struct Leg {
    pub name: (String, String),
    pub safe: String,
    pub planned: String,
    pub speed: f64,
    pub course: Degree,
    pub distance: f64,
    pub variation: Degree,

    pub wind_direction: Degree,
    pub wind_speed: f64,
}

pub fn convert_leg(json_leg: &JSonLeg) -> Leg {
    Leg {
        name: (json_leg.from.clone(), json_leg.to.clone()),
        safe: json_leg.safe.clone(),
        planned: json_leg.planned.clone(),
        speed: json_leg.speed as f64,
        course: (json_leg.course as f64).into(),
        distance: json_leg.distance as f64,
        variation: (json_leg.variation as f64).into(),

        wind_direction: (json_leg.wind_direction as f64).into(),
        wind_speed: json_leg.wind_speed as f64,
    }
}
