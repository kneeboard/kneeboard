use crate::application::Application;
use crate::common::{append_insert_delete, to_number, to_string, IsLast};

use crate::messages::{PlanChange, PlanMessage};
use crate::validation::{
    create_speed_validation, create_wind_speed_validation, nop_validation, validate_course,
    validate_distance, validate_wind_direction,
};

use core::definition::Leg;

use crate::icons::plus_circle;

use web_sys::Event;

use yew::prelude::*;

pub fn legs_html(ctx: &Context<Application>, route_idx: usize, legs: &[Leg]) -> Html {
    let rows = if legs.is_empty() {
        let append_leg = ctx.link().callback(move |_| on_click_append_leg(route_idx));
        html!(
            <tr>
              <td align="middle" colspan="11">
                <button type="button" class="btn btn-link" onclick={append_leg}>
                {plus_circle(32)}
                </button>
              </td>
            </tr>
        )
    } else {
        let rows: Html = legs
            .iter()
            .enumerate()
            .map(|(leg_idx, leg)| leg_html(ctx, (route_idx, leg_idx), legs.is_last(leg_idx), leg))
            .collect();
        rows
    };

    html!(
        <table class="table table-bordered">
        <thead>
          <tr>
            <th>{"From"}</th>
            <th>{"To"}</th>
            <th>{"Safe"}</th>
            <th>{"Planned"}</th>
            <th>{"Speed"}</th>
            <th>{"Course"}</th>
            <th>{"Distance"}</th>
            <th>{"Variation"}</th>
            <th>{"Wind direction"}</th>
            <th>{"Wind speed"}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          { rows }
        </tbody>
        </table>
    )
}

fn leg_html(ctx: &Context<Application>, idx: (usize, usize), is_last: bool, leg: &Leg) -> Html {
    let from = leg.from.clone();
    let to = leg.to.clone();
    let safe = leg.safe.clone();
    let planned = leg.planned.clone();
    let speed = leg.speed.to_string();
    let course = leg.course.to_string();
    let distance = leg.distance.to_string();
    let variation = leg.variation.to_string();
    let wind_direction = leg.wind_direction.to_string();
    let wind_speed = leg.wind_speed.to_string();

    let link = ctx.link();
    let class = "form-control";

    let validate_speed = create_speed_validation(leg.wind_speed);
    let validate_wind_speed = create_wind_speed_validation(leg.speed);

    let from_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegFrom));
    let to_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegTo));
    let safe_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegSafe));
    let planned_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegPlanned));
    let speed_fn =
        link.callback(move |e| on_change_num(idx, e, PlanChange::LegSpeed, &validate_speed));
    let course_fn =
        link.callback(move |e| on_change_num(idx, e, PlanChange::LegCourse, &validate_course));
    let distance_fn =
        link.callback(move |e| on_change_num(idx, e, PlanChange::LegDistance, &validate_distance));
    let variation_fn =
        link.callback(move |e| on_change_num(idx, e, PlanChange::LegVariation, &nop_validation));
    let wind_direction_fn = link.callback(move |e| {
        on_change_num(
            idx,
            e,
            PlanChange::LegWindDirection,
            &validate_wind_direction,
        )
    });
    let wind_speed_fn = link
        .callback(move |e| on_change_num(idx, e, PlanChange::LegWindSpeed, &validate_wind_speed));

    let (route_idx, _) = idx;
    let delete_leg = link.callback(move |_| on_click_delete_leg(idx));
    let insert_leg = link.callback(move |_| on_click_insert_leg(idx));
    let append_leg = link.callback(move |_| on_click_append_leg(route_idx));

    html!(
        <tr>
            <td width="12%"><input type="text" class={class} value={from} onchange={from_fn} /></td>
            <td width="12%"><input type="text" class={class} value={to} onchange={to_fn} /></td>
            <td><input type="text" class={class} value={safe} onchange={safe_fn} /></td>
            <td><input type="text" class={class} value={planned} onchange={planned_fn} /></td>
            <td><input type="number" class={class} value={speed} onchange={speed_fn} /></td>
            <td><input type="number" class={class} value={course} onchange={course_fn} /></td>
            <td><input type="number" class={class} value={distance} onchange={distance_fn} /></td>
            <td><input type="number" class={class} value={variation} onchange={variation_fn} /></td>
            <td><input type="number" class={class} value={wind_direction} onchange={wind_direction_fn} /></td>
            <td><input type="number" class={class} value={wind_speed} onchange={wind_speed_fn} /></td>
            <td valign="top" style="padding: 0px">
                {append_insert_delete(append_leg, insert_leg, delete_leg, is_last) }
            </td>
        </tr>
    )
}

fn on_change_str(
    idx: (usize, usize),
    e: Event,
    func: fn((usize, usize), String) -> PlanChange,
) -> PlanMessage {
    let value = to_string(e);
    PlanMessage::DataChange(func(idx, value))
}

fn on_change_num<F: Fn(i64) -> Option<PlanMessage>>(
    idx: (usize, usize),
    e: Event,
    func: fn((usize, usize), i64) -> PlanChange,
    validate: &F,
) -> PlanMessage {
    let value = to_number(e);

    if let Some(msg) = validate(value) {
        msg
    } else {
        PlanMessage::DataChange(func(idx, value))
    }
}

fn on_click_delete_leg(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::LegDelete(idx))
}

fn on_click_insert_leg(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::LegInsert(idx))
}

fn on_click_append_leg(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::LegAppend(idx))
}
