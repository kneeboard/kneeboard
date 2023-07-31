use pdf::{init_page, ContentBuilder, FontStyle, PDFPageBuilder};

use crate::{
    calc::{calc_aircraft, Degree, Heading, Velocity},
    draw_utils::{disclaimer, write},
};

const MARGIN_SIDE: f64 = 5.;
const MARGIN_TOP: f64 = 30.;

const FONT_SIZE: f64 = 11.;

const LINE_WIDTH: f64 = 0.25;

pub fn create_wind_table(
    builder: &mut PDFPageBuilder,
    air_speed: f64,
    variation: Degree,
    wind: &Velocity,
) {
    let mut layer = builder.content_builder();
    init_page(&mut layer);
    disclaimer(&mut layer);

    let (page_width, _) = layer.page_size();

    let name_height = 5.;

    {
        let wind_bearing = wind.bearing.reciprocal().as_heading();
        let wind_speed = wind.speed.round() as i64;

        let details = format!(
            "Speed:{}, Wind:{}\u{00b0} / {}",
            air_speed, wind_bearing, wind_speed
        );

        let font = (FontStyle::Bold, FONT_SIZE);
        write(&mut layer, &details, (MARGIN_SIDE, 20.), &font)
    }

    let shift = 36.1;
    let left_start = MARGIN_SIDE;
    for (count, n) in (0..90).step_by(5).enumerate() {
        let y = MARGIN_TOP + count as f64 * (name_height + 2.);

        if count % 2 == 0 {
            layer.save_graphics_state();
            layer.set_colour_non_stroking(0.9, 0.90, 0.90);
            layer.set_colour(0., 0., 0.);
            layer.line_width(LINE_WIDTH);

            layer.rectangle((left_start, y - 1.), page_width - (MARGIN_SIDE * 2.), 4.);
            layer.fill();
            layer.restore_graphics_state();
        }

        {
            let destination_bearing = (n as f64).into();
            let heading = calc_aircraft(air_speed, destination_bearing, variation, wind);
            column_line(&mut layer, y, left_start, &heading)
        }

        {
            let destination_bearing = ((n + 90) as f64).into();
            let heading = calc_aircraft(air_speed, destination_bearing, variation, wind);
            column_line(&mut layer, y, left_start + shift, &heading)
        }

        {
            let destination_bearing = ((n + 180) as f64).into();
            let heading = calc_aircraft(air_speed, destination_bearing, variation, wind);
            column_line(&mut layer, y, left_start + shift * 2., &heading)
        }

        {
            let destination_bearing = ((n + 270) as f64).into();
            let heading = calc_aircraft(air_speed, destination_bearing, variation, wind);
            column_line(&mut layer, y, left_start + shift * 3., &heading)
        }

        let distances = [60, 70, 80, 90, 100, 110, 120, 130, 140];
        let speeds = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70];
        dist_time(&mut layer, &distances, &speeds);
    }

    fn column_line(text: &mut ContentBuilder, y: f64, x_offset: f64, heading: &Heading) {
        let font = (FontStyle::Normal, FONT_SIZE);
        let font_bold = (FontStyle::Bold, FONT_SIZE);
        let x = x_offset;

        let name_height = 2.5;
        let track = heading.destination_bearing.as_heading();
        write(text, track.as_str(), (x, y + name_height), &font_bold);

        let speed_overground = heading.speed_overground.round() as i64;
        let fly_heading = heading.heading_magnetic.as_heading();

        write(
            text,
            fly_heading.as_str(),
            (x + 10., y + name_height),
            &font,
        );

        let ground_speed = format!("{}kt", speed_overground);
        write(
            text,
            ground_speed.as_str(),
            (x + 10. + 10., y + name_height),
            &font,
        );
    }
}

pub fn dist_time(layer: &mut ContentBuilder, degrees: &[i32], speed: &[i32]) {
    let (page_width, _) = layer.page_size();

    let _font = (FontStyle::Normal, FONT_SIZE);
    const MARGIN_SIDE: f64 = 5.;
    const MARGIN_TOP: f64 = 171.;

    const FONT_SIZE: f64 = 9.;

    let x = MARGIN_SIDE;
    let y = MARGIN_TOP;

    let width_inc = (page_width - (MARGIN_SIDE * 2.)) / (speed.len() as f64 + 1.);

    {
        layer.save_graphics_state();
        layer.set_colour(0., 0., 0.);
        layer.line_width(LINE_WIDTH);

        let point = (x - 1., (y - 4.));
        let width = page_width - (MARGIN_SIDE * 2.);
        let h = (degrees.len() as f64) * 4.6;

        layer.rectangle(point, width, h);
        layer.stroke_path();
        layer.restore_graphics_state();
    }

    for (idx, value) in speed.iter().enumerate() {
        let x_pos = width_inc + x + (idx as f64 * width_inc);
        let font = (FontStyle::Bold, 11.);
        let location = (x_pos, y);
        write(layer, &value.to_string(), location, &font);
    }

    for (y_step, degree) in degrees.iter().enumerate() {
        let height = y + (y_step as f64 * 4.) + 3.8;
        if y_step % 2 == 0 {
            let hieght = 4.;

            let point = (x, height - 3.);
            let width = page_width - (MARGIN_SIDE * 2.) - 2.;

            layer.save_graphics_state();
            layer.set_colour_non_stroking(0.9, 0.90, 0.90);

            layer.rectangle(point, width, hieght);
            layer.fill();
            layer.restore_graphics_state();
        }

        let location = (x, height);
        let font = (FontStyle::Bold, FONT_SIZE);
        write(layer, &degree.to_string(), location, &font);

        for (idx, value) in speed.iter().enumerate() {
            let x_pos = width_inc + x + (idx as f64 * width_inc);

            let cross_wind = calc_time(*degree, *value);

            let location = (x_pos, height);
            let font = (FontStyle::Bold, FONT_SIZE);
            write(layer, &cross_wind.to_string(), location, &font);
        }
    }
}

fn calc_time(speed: i32, distance: i32) -> i32 {
    (60 * distance) / speed
}
