use definition::Hold as HoldDef;
use pdf::{init_page, ContentBuilder, FontStyle, PDFPageBuilder};

use crate::{
    calc::{calc_aircraft, Degree, Velocity},
    draw_utils::{disclaimer, write},
};

// Geometry constants matching ppl_nav exactly (local space in mm)
const SCALE: f64 = 10.0;
const LINE_LENGTH: f64 = 3.4 * SCALE; // = 34.0
const A: f64 = 1.00005519 * SCALE; // ≈ 10.0006  (bezier approx constants)
const B: f64 = 0.55342686 * SCALE; // ≈ 5.5343
const C_K: f64 = 0.99873585 * SCALE; // ≈ 9.9874

// Display scale: matches ppl_nav's CurTransMat::Scale(2., 2.)
const DISP: f64 = 2.0;

// Beacon page position in screen mm (y-down from top-left).
// ppl_nav's cm matrix is compose(Rotate(0), Translate(20,50), Scale(2,2)).
// After composing, local (0,0) → page (20*2=40mm, 50*2=100mm from bottom).
// screen_y = 210 - 100 = 110mm from top.
const BX: f64 = 40.0;
const BY: f64 = 110.0;

// Line width matching ppl_nav's set_outline_thickness(0.5) in PDF points.
// 0.5pt × (25.4mm/72pt) ≈ 0.176mm
const LW: f64 = 0.5 * 25.4 / 72.0;

/// Convert ppl_nav local coords (beacon at origin, y-up) to kneeboard screen mm (y-down from top).
#[inline]
fn lp(lx: f64, ly: f64) -> (f64, f64) {
    (BX + lx * DISP, BY - ly * DISP)
}

pub fn create_hold(builder: &mut PDFPageBuilder, hold: &HoldDef) {
    let mut layer = builder.content_builder();
    init_page(&mut layer);
    disclaimer(&mut layer);

    let right_hand = hold.right_hand;
    let in_bound_track = Degree::new(hold.in_bound_track as f64);
    let variation = Degree::new(hold.variation as f64);
    let wind = Velocity {
        speed: hold.wind.speed as f64,
        // meteorological wind direction → bearing vector (direction wind blows TO)
        bearing: Degree::new(hold.wind.angle as f64).reciprocal(),
    };
    let air_speed = hold.aircraft_speed as f64;

    // offset_y: hold is above track for right-hand (+SCALE), below for left-hand (-SCALE)
    let oy = if right_hand { SCALE } else { -SCALE };

    draw_racetrack(&mut layer, oy);
    draw_beacon(&mut layer);
    draw_inbound_line(&mut layer);
    draw_divide_line(&mut layer, right_hand);
    draw_gate_line(&mut layer, right_hand);
    draw_ten_deg_tick(&mut layer, right_hand);

    draw_labels(
        &mut layer,
        right_hand,
        in_bound_track,
        variation,
        &wind,
        air_speed,
        &hold.description,
        hold.wind.speed,
        hold.aircraft_speed,
    );
}

/// Draw the racetrack oval. oy = +SCALE (right-hand) or -SCALE (left-hand).
/// The racetrack has:
///   right semicircle centred at (LINE_LENGTH, oy)
///   left  semicircle centred at (0,           oy)
///   straight edges at y = oy ± A  (inner/track edge and outer edge)
fn draw_racetrack(layer: &mut ContentBuilder, oy: f64) {
    let ox = LINE_LENGTH; // = 34

    layer.save_graphics_state();
    layer.line_width(LW);

    // Start at top of right semicircle
    layer.begin_subpath(lp(ox, A + oy));
    // Right semicircle: top → rightmost  (quarter-circle via bezier)
    layer.curve_to(lp(B + ox, C_K + oy), lp(C_K + ox, B + oy), lp(A + ox, oy));
    // Right semicircle: rightmost → bottom  (quarter-circle)
    layer.curve_to(
        lp(C_K + ox, -B + oy),
        lp(B + ox, -C_K + oy),
        lp(ox, -A + oy),
    );
    // Bottom straight edge: right → left (back to beacon side)
    layer.line(lp(0.0, -A + oy));
    // Left semicircle: bottom → leftmost  (quarter-circle)
    layer.curve_to(lp(-B, -A + oy), lp(-C_K, -B + oy), lp(-A, oy));
    // Left semicircle: leftmost → top  (quarter-circle)
    layer.curve_to(lp(-A, B + oy), lp(-B, C_K + oy), lp(0.0, A + oy));
    // Top straight edge: left → right
    layer.line(lp(ox, A + oy));
    layer.close_path();
    layer.stroke_path();
    layer.restore_graphics_state();
}

/// Small square at beacon (local origin).
fn draw_beacon(layer: &mut ContentBuilder) {
    let bs = 0.25 * SCALE; // half-size = 1.25mm local → 2.5mm displayed
    layer.save_graphics_state();
    layer.line_width(LW);
    layer.begin_subpath(lp(0.5 * bs, 0.5 * bs));
    layer.line(lp(0.5 * bs, -0.5 * bs));
    layer.line(lp(-0.5 * bs, -0.5 * bs));
    layer.line(lp(-0.5 * bs, 0.5 * bs));
    layer.close_path();
    layer.stroke_path();
    layer.restore_graphics_state();
}

/// Inbound track line passing through the beacon.
fn draw_inbound_line(layer: &mut ContentBuilder) {
    layer.save_graphics_state();
    layer.line_width(LW);
    layer.begin_subpath(lp(-15.0, 0.0));
    layer.line(lp(LINE_LENGTH * 1.5, 0.0));
    layer.stroke_path();
    layer.restore_graphics_state();
}

/// Dividing line separating the offset-entry sector from the parallel-entry sector.
/// Ported directly from ppl_nav's divide logic.
fn draw_divide_line(layer: &mut ContentBuilder, right_hand: bool) {
    let (top, bottom) = if right_hand {
        (3.0_f64, 5.0_f64)
    } else {
        (5.0, 3.0)
    };
    let flip: f64 = if right_hand { 1.0 } else { -1.0 };

    let x0 = flip * -LINE_LENGTH / bottom;
    let y0 = -(LINE_LENGTH / bottom) * 2.75;
    let x1 = flip * LINE_LENGTH / top;
    let y1 = (LINE_LENGTH / top) * 2.75;

    layer.save_graphics_state();
    layer.line_width(LW);
    layer.begin_subpath(lp(x0, y0));
    layer.line(lp(x1, y1));
    layer.stroke_path();
    layer.restore_graphics_state();
}

/// Gate entry line at 30° from inbound track (tan 30° ≈ 0.577).
fn draw_gate_line(layer: &mut ContentBuilder, right_hand: bool) {
    let flip: f64 = if right_hand { 1.0 } else { -1.0 };
    layer.save_graphics_state();
    layer.line_width(LW);
    layer.begin_subpath(lp(0.0, 0.0));
    layer.line(lp(LINE_LENGTH * 1.2, flip * LINE_LENGTH * 1.2 * 0.577));
    layer.stroke_path();
    layer.restore_graphics_state();
}

/// Short tick mark at 10° from inbound track (tan 10° ≈ 0.176).
fn draw_ten_deg_tick(layer: &mut ContentBuilder, right_hand: bool) {
    let flip: f64 = if right_hand { 1.0 } else { -1.0 };
    layer.save_graphics_state();
    layer.line_width(LW);
    layer.begin_subpath(lp(LINE_LENGTH * 1.25, flip * LINE_LENGTH * 1.25 * 0.176));
    layer.line(lp(LINE_LENGTH * 1.32, flip * LINE_LENGTH * 1.32 * 0.176));
    layer.stroke_path();
    layer.restore_graphics_state();
}

#[allow(clippy::too_many_arguments)]
fn draw_labels(
    layer: &mut ContentBuilder,
    right_hand: bool,
    in_bound_track: Degree,
    variation: Degree,
    wind: &Velocity,
    air_speed: f64,
    description: &str,
    wind_speed: i64,
    aircraft_speed: i64,
) {
    let font = (FontStyle::Normal, 7.0_f64);
    let font_hdr = (FontStyle::Normal, 12.0_f64);

    // Wind and speed header — absolute screen position (matches ppl_nav Mm(5.), Mm(185.))
    {
        let wind_dir = wind.bearing.reciprocal();
        let text = format!(
            "Wind: {}@{}kt  Speed: {}kt",
            wind_dir.as_heading(),
            wind_speed,
            aircraft_speed
        );
        write(layer, &text, (5.0, 25.0), &font_hdr);
    }

    // Description — absolute screen position (matches ppl_nav Mm(5.), Mm(180.))
    write(layer, description, (5.0, 35.0), &font_hdr);

    // Inbound track label
    {
        let (trk, hdg, gs, t) = calc_inbound(air_speed, in_bound_track, variation, wind);
        let text = format!("{} ({}) {}kt [{}]", trk, hdg, gs, t);
        write(layer, &text, lp(LINE_LENGTH / 3.8, 1.0), &font);
    }

    // Outbound track label (triple WCA heading)
    {
        let out_track = in_bound_track.reciprocal();
        let (trk, hdg, gs, t) = calc_outbound(air_speed, out_track, variation, wind);
        let text = format!("{} ({}) {}kt [{}]", trk, hdg, gs, t);
        let local_y = if right_hand { 21.0 } else { -22.5 };
        write(layer, &text, lp(LINE_LENGTH / 3.8, local_y), &font);
    }

    // Gate entry heading label (30° sector boundary)
    {
        let gate_track = if right_hand {
            Degree::new(in_bound_track.degrees - 30.).reciprocal()
        } else {
            Degree::new(in_bound_track.degrees + 30.).reciprocal()
        };
        let (trk, hdg, gs, t) = calc_inbound(air_speed, gate_track, variation, wind);
        let text = format!("{} ({}) {}kt [{}]", trk, hdg, gs, t);
        let local_y = if right_hand { 26.0 } else { -26.5 };
        write(layer, &text, lp(LINE_LENGTH * 0.8, local_y), &font);
    }

    // 10-degree sector label
    {
        let ten_deg = if right_hand {
            Degree::new(in_bound_track.degrees - 60.)
        } else {
            Degree::new(in_bound_track.degrees + 60.)
        };
        let local_y = if right_hand { 7.0 } else { -8.5 };
        write(
            layer,
            &ten_deg.as_heading(),
            lp(LINE_LENGTH * 1.35, local_y),
            &font,
        );
    }

    // Outbound track number on inbound line extension (left of beacon)
    {
        let out_bound = in_bound_track.reciprocal();
        write(layer, &out_bound.as_heading(), lp(-15.0, 1.0), &font);
    }

    // Divide sector boundary headings
    {
        let adjust = if right_hand { -70.0 } else { 70.0 };
        let div1 = Degree::new(in_bound_track.degrees + adjust).reciprocal();
        let (ly1, ly2) = if right_hand {
            (-21.0, 32.0)
        } else {
            (20.0, -35.0)
        };
        write(layer, &div1.as_heading(), lp(-10.0, ly1), &font);
        let div2 = Degree::new(in_bound_track.degrees + adjust);
        write(layer, &div2.as_heading(), lp(10.0, ly2), &font);
    }

    // Entry sector labels
    let (oe_ly, pe_ly, de1_ly, de2_ly) = if right_hand {
        (-10.0, 20.0, 30.0, -15.0)
    } else {
        (10.0, -20.0, -30.0, 15.0)
    };
    write(layer, "OE", lp(-15.0, oe_ly), &font);
    write(layer, "PE", lp(-15.0, pe_ly), &font);
    write(layer, "DE", lp(LINE_LENGTH, de1_ly), &font);
    write(layer, "DE", lp(LINE_LENGTH * 0.5, de2_ly), &font);
}

fn calc_inbound(
    air_speed: f64,
    track: Degree,
    variation: Degree,
    wind: &Velocity,
) -> (String, String, i64, String) {
    let result = calc_aircraft(air_speed, track, variation, wind);
    let gs = result.speed_overground.round() as i64;
    let time_secs = ((air_speed / 60.) / result.speed_overground * 3600.) as i64;
    (
        track.as_heading(),
        result.heading_magnetic.as_heading(),
        gs,
        to_time(time_secs),
    )
}

fn calc_outbound(
    air_speed: f64,
    track: Degree,
    variation: Degree,
    wind: &Velocity,
) -> (String, String, i64, String) {
    let result = calc_aircraft(air_speed, track, variation, wind);
    // Triple wind-correction angle on the outbound leg
    let triple_wca = Degree::new(result.correction_angle.degrees * 3.);
    let hdg_magnetic = track + triple_wca + variation;
    let gs = result.speed_overground.round() as i64;
    let time_secs = ((air_speed / 60.) / result.speed_overground * 3600.) as i64;
    (
        track.as_heading(),
        hdg_magnetic.as_heading(),
        gs,
        to_time(time_secs),
    )
}

fn to_time(secs: i64) -> String {
    let s = secs.abs();
    format!("{}:{:02}", s / 60, s % 60)
}
