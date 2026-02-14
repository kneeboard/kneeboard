use crate::application::Application;
use crate::common::{to_number, IsLast};
use crate::messages::{PlanChange, PlanMessage};

use definition::Diversion;

use web_sys::Event;

use yew::prelude::*;

pub fn diversion_html(ctx: &Context<Application>, diversions: &[Diversion]) -> Html {
    let diversions_html: Html = diversions
        .iter()
        .enumerate()
        .map(|(idx, div)| diversion(ctx, div, idx, diversions.is_last(idx)))
        .collect();

    let append = ctx.link().callback(move |_| on_change_append());

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Diversions"}
                </div>
            </div>
            <div class="panel-body" style="padding:8px 14px;">
                <table>
                    <thead>
                        <tr>
                            <th class="ra">{"TAS"}</th>
                            <th class="ra">{"VAR"}</th>
                            <th class="ra">{"W/DIR"}</th>
                            <th class="ra">{"W/SPD"}</th>
                            <th style="width:68px"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {diversions_html}
                    </tbody>
                </table>
                <button class="add-row" onclick={append}>{"+ Diversion"}</button>
            </div>
        </div>
    )
}

fn on_change_append() -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionAppend)
}

fn diversion(
    ctx: &Context<Application>,
    diversion: &Diversion,
    idx: usize,
    _is_last: bool,
) -> Html {
    let link = ctx.link();
    let speed_callback = link.callback(move |e| on_change(e, idx, PlanChange::DiversionSpeed));
    let variation_callback =
        link.callback(move |e| on_change(e, idx, PlanChange::DiversionVariation));
    let wind_direction_callback =
        link.callback(move |e| on_change(e, idx, PlanChange::DiversionWindDirection));
    let wind_speed_callback =
        link.callback(move |e| on_change(e, idx, PlanChange::DiversionWindSpeed));

    let speed = diversion.aircraft_speed;
    let variation = diversion.variation;
    let wind_direction = diversion.wind.angle;
    let wind_speed = diversion.wind.speed;

    let delete = ctx.link().callback(move |_| on_change_delete(idx));

    html!(
        <tr>
          <td><input type="number" class="ra" value={speed.to_string()} onchange={speed_callback}/></td>
          <td><input type="number" class="ra" value={variation.to_string()} onchange={variation_callback}/></td>
          <td><input type="number" class="ra" value={wind_direction.to_string()} onchange={wind_direction_callback}/></td>
          <td><input type="number" class="ra" value={wind_speed.to_string()} onchange={wind_speed_callback}/></td>
          <td>
            <div class="row-acts" style="opacity:1">
                <button class="ibtn del" onclick={delete} title="Delete diversion">{"×"}</button>
            </div>
          </td>
        </tr>
    )
}

#[allow(dead_code)]
fn on_change_insert(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionInsert(idx))
}

fn on_change_delete(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::DiversionDelete(idx))
}

fn on_change(e: Event, idx: usize, func: fn(usize, i64) -> PlanChange) -> PlanMessage {
    let value = to_number(e);
    PlanMessage::DataChange(func(idx, value))
}
