use pdf::{init_page, ContentBuilder, FontStyle, PDFPageBuilder};

use crate::{
    calc::{calc_aircraft, Degree, Velocity},
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
    let rows = calc_diversions(air_speed, variation, wind);

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
    for (count, [h1, h2, h3, h4]) in rows.iter().enumerate() {
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

        column_line(&mut layer, y, left_start, h1);
        column_line(&mut layer, y, left_start + shift, h2);
        column_line(&mut layer, y, left_start + shift * 2., h3);
        column_line(&mut layer, y, left_start + shift * 3., h4);

        let distances = [60, 70, 80, 90, 100, 110, 120, 130, 140];
        let speeds = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70];
        dist_time(&mut layer, &distances, &speeds);
    }

    fn column_line(
        text: &mut ContentBuilder,
        y: f64,
        x_offset: f64,
        Diversion {
            course: track,
            heading: fly_heading,
            ground_speed: speed_overground,
        }: &Diversion,
    ) {
        let font = (FontStyle::Normal, FONT_SIZE);
        let font_bold = (FontStyle::Bold, FONT_SIZE);
        let x = x_offset;

        let name_height = 2.5;
        write(
            text,
            &format!("{track}\u{00b0}"),
            (x, y + name_height),
            &font_bold,
        );

        write(
            text,
            &format!("{fly_heading}\u{00b0}"),
            (x + 10., y + name_height),
            &font,
        );

        let ground_speed = format!("{speed_overground}kt");
        write(
            text,
            ground_speed.as_str(),
            (x + 10. + 10., y + name_height),
            &font,
        );
    }
}

fn calc_diversions(air_speed: f64, variation: Degree, wind: &Velocity) -> Vec<[Diversion; 4]> {
    let mut rows = vec![];
    for n in (0..90).step_by(5) {
        let h1 = calc_diversion(n as f64, air_speed, variation, wind);
        let h2 = calc_diversion((n + 90) as f64, air_speed, variation, wind);
        let h3 = calc_diversion((n + 180) as f64, air_speed, variation, wind);
        let h4 = calc_diversion((n + 270) as f64, air_speed, variation, wind);

        rows.push([h1, h2, h3, h4]);
    }

    rows
}

fn calc_diversion(course: f64, air_speed: f64, variation: Degree, wind: &Velocity) -> Diversion {
    let destination_bearing = course.into();
    let diversion = calc_aircraft(air_speed, destination_bearing, variation, wind);

    if diversion.speed_overground >= 0. && diversion.speed_overground.is_finite() {
        let course = diversion.destination_bearing.as_heading();
        let heading = diversion.heading_magnetic.as_heading();
        let ground_speed = (diversion.speed_overground.round() as i64).to_string();
        Diversion::new(course, heading, ground_speed)
    } else {
        Diversion::new("---".to_owned(), "---".to_owned(), "---".to_owned())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Diversion {
    course: String,
    heading: String,
    ground_speed: String,
}

impl Diversion {
    fn new(course: String, heading: String, ground_speed: String) -> Self {
        Diversion {
            course,
            heading,
            ground_speed,
        }
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

#[cfg(test)]
mod tests {
    use super::{calc_diversions, Diversion};
    use crate::calc::{Degree, Velocity};

    #[test]
    pub fn regression() {
        let air_speed = 100_f64;
        let variation: Degree = 1_f64.into();

        let wind_bearing: Degree = 310_f64.into();
        let wind = Velocity {
            speed: 20.,
            bearing: wind_bearing.reciprocal(),
        };

        let actual = calc_diversions(air_speed, variation, &wind);

        let expected = vec![
            [
                Diversion::new("000".to_owned(), "352".to_owned(), "86".to_owned()),
                Diversion::new("090".to_owned(), "084".to_owned(), "114".to_owned()),
                Diversion::new("180".to_owned(), "190".to_owned(), "112".to_owned()),
                Diversion::new("270".to_owned(), "278".to_owned(), "84".to_owned()),
            ],
            [
                Diversion::new("005".to_owned(), "357".to_owned(), "87".to_owned()),
                Diversion::new("095".to_owned(), "089".to_owned(), "116".to_owned()),
                Diversion::new("185".to_owned(), "195".to_owned(), "110".to_owned()),
                Diversion::new("275".to_owned(), "283".to_owned(), "83".to_owned()),
            ],
            [
                Diversion::new("010".to_owned(), "001".to_owned(), "88".to_owned()),
                Diversion::new("100".to_owned(), "095".to_owned(), "117".to_owned()),
                Diversion::new("190".to_owned(), "201".to_owned(), "108".to_owned()),
                Diversion::new("280".to_owned(), "287".to_owned(), "82".to_owned()),
            ],
            [
                Diversion::new("015".to_owned(), "006".to_owned(), "90".to_owned()),
                Diversion::new("105".to_owned(), "101".to_owned(), "118".to_owned()),
                Diversion::new("195".to_owned(), "206".to_owned(), "107".to_owned()),
                Diversion::new("285".to_owned(), "291".to_owned(), "82".to_owned()),
            ],
            [
                Diversion::new("020".to_owned(), "010".to_owned(), "91".to_owned()),
                Diversion::new("110".to_owned(), "107".to_owned(), "119".to_owned()),
                Diversion::new("200".to_owned(), "212".to_owned(), "105".to_owned()),
                Diversion::new("290".to_owned(), "295".to_owned(), "81".to_owned()),
            ],
            [
                Diversion::new("025".to_owned(), "015".to_owned(), "93".to_owned()),
                Diversion::new("115".to_owned(), "113".to_owned(), "119".to_owned()),
                Diversion::new("205".to_owned(), "217".to_owned(), "103".to_owned()),
                Diversion::new("295".to_owned(), "299".to_owned(), "81".to_owned()),
            ],
            [
                Diversion::new("030".to_owned(), "020".to_owned(), "95".to_owned()),
                Diversion::new("120".to_owned(), "119".to_owned(), "120".to_owned()),
                Diversion::new("210".to_owned(), "222".to_owned(), "102".to_owned()),
                Diversion::new("300".to_owned(), "303".to_owned(), "80".to_owned()),
            ],
            [
                Diversion::new("035".to_owned(), "025".to_owned(), "96".to_owned()),
                Diversion::new("125".to_owned(), "125".to_owned(), "120".to_owned()),
                Diversion::new("215".to_owned(), "227".to_owned(), "100".to_owned()),
                Diversion::new("305".to_owned(), "307".to_owned(), "80".to_owned()),
            ],
            [
                Diversion::new("040".to_owned(), "029".to_owned(), "98".to_owned()),
                Diversion::new("130".to_owned(), "131".to_owned(), "120".to_owned()),
                Diversion::new("220".to_owned(), "233".to_owned(), "98".to_owned()),
                Diversion::new("310".to_owned(), "311".to_owned(), "80".to_owned()),
            ],
            [
                Diversion::new("045".to_owned(), "035".to_owned(), "100".to_owned()),
                Diversion::new("135".to_owned(), "137".to_owned(), "120".to_owned()),
                Diversion::new("225".to_owned(), "237".to_owned(), "96".to_owned()),
                Diversion::new("315".to_owned(), "315".to_owned(), "80".to_owned()),
            ],
            [
                Diversion::new("050".to_owned(), "040".to_owned(), "102".to_owned()),
                Diversion::new("140".to_owned(), "143".to_owned(), "120".to_owned()),
                Diversion::new("230".to_owned(), "242".to_owned(), "95".to_owned()),
                Diversion::new("320".to_owned(), "319".to_owned(), "80".to_owned()),
            ],
            [
                Diversion::new("055".to_owned(), "045".to_owned(), "103".to_owned()),
                Diversion::new("145".to_owned(), "149".to_owned(), "119".to_owned()),
                Diversion::new("235".to_owned(), "247".to_owned(), "93".to_owned()),
                Diversion::new("325".to_owned(), "323".to_owned(), "81".to_owned()),
            ],
            [
                Diversion::new("060".to_owned(), "050".to_owned(), "105".to_owned()),
                Diversion::new("150".to_owned(), "155".to_owned(), "119".to_owned()),
                Diversion::new("240".to_owned(), "252".to_owned(), "91".to_owned()),
                Diversion::new("330".to_owned(), "327".to_owned(), "81".to_owned()),
            ],
            [
                Diversion::new("065".to_owned(), "056".to_owned(), "107".to_owned()),
                Diversion::new("155".to_owned(), "161".to_owned(), "118".to_owned()),
                Diversion::new("245".to_owned(), "256".to_owned(), "90".to_owned()),
                Diversion::new("335".to_owned(), "331".to_owned(), "82".to_owned()),
            ],
            [
                Diversion::new("070".to_owned(), "061".to_owned(), "108".to_owned()),
                Diversion::new("160".to_owned(), "167".to_owned(), "117".to_owned()),
                Diversion::new("250".to_owned(), "261".to_owned(), "88".to_owned()),
                Diversion::new("340".to_owned(), "335".to_owned(), "82".to_owned()),
            ],
            [
                Diversion::new("075".to_owned(), "067".to_owned(), "110".to_owned()),
                Diversion::new("165".to_owned(), "173".to_owned(), "116".to_owned()),
                Diversion::new("255".to_owned(), "265".to_owned(), "87".to_owned()),
                Diversion::new("345".to_owned(), "339".to_owned(), "83".to_owned()),
            ],
            [
                Diversion::new("080".to_owned(), "072".to_owned(), "112".to_owned()),
                Diversion::new("170".to_owned(), "178".to_owned(), "114".to_owned()),
                Diversion::new("260".to_owned(), "270".to_owned(), "86".to_owned()),
                Diversion::new("350".to_owned(), "344".to_owned(), "84".to_owned()),
            ],
            [
                Diversion::new("085".to_owned(), "078".to_owned(), "113".to_owned()),
                Diversion::new("175".to_owned(), "184".to_owned(), "113".to_owned()),
                Diversion::new("265".to_owned(), "274".to_owned(), "85".to_owned()),
                Diversion::new("355".to_owned(), "348".to_owned(), "85".to_owned()),
            ],
        ];

        assert_eq!(expected, actual);
    }
}
