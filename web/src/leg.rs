use crate::application::Application;
use crate::common::{to_number, to_string, IsLast};

use crate::messages::{PlanChange, PlanMessage};

use definition::Leg;

use web_sys::Event;

use yew::prelude::*;

pub fn legs_html(ctx: &Context<Application>, route_idx: usize, legs: &[Leg]) -> Html {
    let rows = if legs.is_empty() {
        html!()
    } else {
        let rows: Html = legs
            .iter()
            .enumerate()
            .map(|(leg_idx, leg)| leg_html(ctx, (route_idx, leg_idx), legs.is_last(leg_idx), leg))
            .collect();
        rows
    };

    let append_leg = ctx.link().callback(move |_| on_click_append_leg(route_idx));
    let fill_row = fill_row_html(ctx, route_idx);

    html!(
        <>
        <table>
        <thead>
          <tr>
            <th>{"From"}</th>
            <th>{"To"}</th>
            <th>{"Safe"}</th>
            <th>{"Plan"}</th>
            <th class="ra">{"TAS"}</th>
            <th class="ra">{"CRS"}</th>
            <th class="ra">{"DST"}</th>
            <th class="ra">{"VAR"}</th>
            <th class="ra">{"W/D"}</th>
            <th class="ra">{"W/S"}</th>
            <th style="width:68px"></th>
          </tr>
          { fill_row }
        </thead>
        <tbody>
          { rows }
        </tbody>
        </table>
        <button class="add-row" onclick={append_leg}>{"+ Leg"}</button>
        </>
    )
}

fn fill_row_html(ctx: &Context<Application>, route_idx: usize) -> Html {
    let link = ctx.link();

    let safe_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillSafe(route_idx, to_string(e)))
    });
    let planned_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillPlanned(route_idx, to_string(e)))
    });
    let speed_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillSpeed(route_idx, parse_fill_number(e)))
    });
    let course_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillCourse(route_idx, parse_fill_number(e)))
    });
    let distance_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillDistance(
            route_idx,
            parse_fill_number(e),
        ))
    });
    let variation_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillVariation(
            route_idx,
            parse_fill_number(e),
        ))
    });
    let wind_dir_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillWindDirection(
            route_idx,
            parse_fill_number(e),
        ))
    });
    let wind_spd_fn = link.callback(move |e: Event| {
        PlanMessage::DataChange(PlanChange::RouteFillWindSpeed(
            route_idx,
            parse_fill_number(e),
        ))
    });

    html!(
        <tr class="fill-row">
            <td colspan="2" style="padding: 2px 6px; font-size:10px; color:var(--text-faint); white-space:nowrap;">{"fill all ↓"}</td>
            <td><input class="fill-input" type="text"   placeholder="—" onchange={safe_fn} /></td>
            <td><input class="fill-input" type="text"   placeholder="—" onchange={planned_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={speed_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={course_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={distance_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={variation_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={wind_dir_fn} /></td>
            <td><input class="fill-input ra" type="number" placeholder="—" onchange={wind_spd_fn} /></td>
            <td></td>
        </tr>
    )
}

fn leg_html(ctx: &Context<Application>, idx: (usize, usize), _is_last: bool, leg: &Leg) -> Html {
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

    let from_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegFrom));
    let to_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegTo));
    let safe_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegSafe));
    let planned_fn = link.callback(move |e| on_change_str(idx, e, PlanChange::LegPlanned));
    let speed_fn = link.callback(move |e| on_change_num(idx, e, PlanChange::LegSpeed));
    let course_fn = link.callback(move |e| on_change_num(idx, e, PlanChange::LegCourse));
    let distance_fn = link.callback(move |e| on_change_num(idx, e, PlanChange::LegDistance));
    let variation_fn = link.callback(move |e| on_change_num(idx, e, PlanChange::LegVariation));
    let wind_direction_fn =
        link.callback(move |e| on_change_num(idx, e, PlanChange::LegWindDirection));
    let wind_speed_fn = link.callback(move |e| on_change_num(idx, e, PlanChange::LegWindSpeed));

    let delete_leg = link.callback(move |_| on_click_delete_leg(idx));
    let insert_leg = link.callback(move |_| on_click_insert_leg(idx));

    html!(
        <tr>
            <td><input type="text" value={from} onchange={from_fn} /></td>
            <td><input type="text" value={to} onchange={to_fn} /></td>
            <td><input type="text" value={safe} onchange={safe_fn} /></td>
            <td><input type="text" value={planned} onchange={planned_fn} /></td>
            <td><input type="number" class="ra" value={speed} onchange={speed_fn} /></td>
            <td><input type="number" class="ra" value={course} onchange={course_fn} /></td>
            <td><input type="number" class="ra" value={distance} onchange={distance_fn} /></td>
            <td><input type="number" class="ra" value={variation} onchange={variation_fn} /></td>
            <td><input type="number" class="ra" value={wind_direction} onchange={wind_direction_fn} /></td>
            <td><input type="number" class="ra" value={wind_speed} onchange={wind_speed_fn} /></td>
            <td>
                <div class="row-acts">
                    <button class="ibtn" onclick={insert_leg} title="Insert leg">{"↥"}</button>
                    <button class="ibtn del" onclick={delete_leg} title="Delete leg">{"×"}</button>
                </div>
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

fn on_change_num(
    idx: (usize, usize),
    e: Event,
    func: fn((usize, usize), i64) -> PlanChange,
) -> PlanMessage {
    let value = to_number(e);
    PlanMessage::DataChange(func(idx, value))
}

fn parse_fill_number(e: Event) -> i64 {
    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
    input.value().trim().parse().unwrap_or(0)
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
