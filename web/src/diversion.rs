use crate::application::Application;
use crate::common::{append_insert_delete, to_number, IsLast};
use crate::messages::{PlanChange, PlanMessage};
use crate::validation::{
    create_speed_validation, create_wind_speed_validation, nop_validation, validate_wind_direction,
};

use core::definition::Diversion;

use crate::icons::chevron_bar_down;

use web_sys::Event;

use yew::prelude::*;

pub fn diversion_html(ctx: &Context<Application>, diversions: &[Diversion]) -> Html {
    let diversions_html = if diversions.is_empty() {
        let append = ctx.link().callback(move |_| on_change_append());
        html!(
            <tr>
              <td colspan="5" align="middle">
                <button type="button" class="btn btn-link btn-sm py-0 px-0" onclick={append}>
                    {chevron_bar_down(32)}
                </button>
              </td>
            </tr>
        )
    } else {
        let diversions_html: Html = diversions
            .iter()
            .enumerate()
            .map(|(idx, div)| diversion(ctx, div, idx, diversions.is_last(idx)))
            .collect();
        diversions_html
    };

    html!(
        <table class="table table-bordered">
          <thead>
            <tr>
              <th>{"Speed"}</th>
              <th>{"Variation"}</th>
              <th>{"Wind Direction"}</th>
              <th>{"Wind Speed"}</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {diversions_html}
          </tbody>
        </table>
    )
}

fn on_change_append() -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionAppend)
}

fn diversion(ctx: &Context<Application>, diversion: &Diversion, idx: usize, is_last: bool) -> Html {
    let validate_speed = create_speed_validation(diversion.wind.speed);
    let validate_wind_speed = create_wind_speed_validation(diversion.aircraft_speed);

    let link = ctx.link();
    let speed_callback =
        link.callback(move |e| on_change(e, idx, PlanChange::DiversionSpeed, &validate_speed));
    let variation_callback =
        link.callback(move |e| on_change(e, idx, PlanChange::DiversionVariation, &nop_validation));
    let wind_direction_callback = link.callback(move |e| {
        on_change(
            e,
            idx,
            PlanChange::DiversionWindDirection,
            &validate_wind_direction,
        )
    });
    let wind_speed_callback = link
        .callback(move |e| on_change(e, idx, PlanChange::DiversionWindSpeed, &validate_wind_speed));

    let speed = diversion.aircraft_speed;
    let variation = diversion.variation;
    let wind_direction = diversion.wind.angle;
    let wind_speed = diversion.wind.speed;

    let insert = ctx.link().callback(move |_| on_change_insert(idx));
    let delete = ctx.link().callback(move |_| on_change_delete(idx));
    let append = ctx.link().callback(move |_| on_change_append());

    html!(
        <tr>
          <td><input type="number" class="form-control" value={speed.to_string()} onchange={speed_callback}/></td>
          <td><input type="number" class="form-control" value={variation.to_string()} onchange={variation_callback}/></td>
          <td><input type="number" class="form-control" value={wind_direction.to_string()} onchange={wind_direction_callback}/></td>
          <td><input type="number" class="form-control" value={wind_speed.to_string()} onchange={wind_speed_callback}/></td>
          <td valign="top" style="padding: 0px" width="5px">
            {append_insert_delete(append, insert, delete, is_last) }
          </td>
        </tr>
    )
}

fn on_change_insert(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionInsert(idx))
}

fn on_change_delete(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionDelete(idx))
}

fn on_change<F: Fn(i64) -> Option<PlanMessage>>(
    e: Event,
    idx: usize,
    func: fn(usize, i64) -> PlanChange,
    validate: &F,
) -> PlanMessage {
    let value = to_number(e);

    if let Some(msg) = validate(value) {
        msg
    } else {
        PlanMessage::DataChange(func(idx, value))
    }
}
