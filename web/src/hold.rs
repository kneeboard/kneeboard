use crate::application::Application;
use crate::common::to_number;
use crate::messages::{PlanChange, PlanMessage};

use definition::Hold;

use web_sys::Event;

use yew::prelude::*;

pub fn hold_html(ctx: &Context<Application>, holds: &[Hold]) -> Html {
    let holds_html: Html = holds
        .iter()
        .enumerate()
        .map(|(idx, h)| hold_row(ctx, h, idx))
        .collect();

    let append = ctx
        .link()
        .callback(move |_| PlanMessage::DataChange(PlanChange::HoldAppend));

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Holds"}
                </div>
            </div>
            <div class="panel-body" style="padding:8px 14px;">
                <table>
                    <thead>
                        <tr>
                            <th>{"Description"}</th>
                            <th class="ra">{"RH"}</th>
                            <th class="ra">{"IBT"}</th>
                            <th class="ra">{"TAS"}</th>
                            <th class="ra">{"VAR"}</th>
                            <th class="ra">{"W/DIR"}</th>
                            <th class="ra">{"W/SPD"}</th>
                            <th style="width:36px"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {holds_html}
                    </tbody>
                </table>
                <button class="add-row" onclick={append}>{"+ Hold"}</button>
            </div>
        </div>
    )
}

fn hold_row(ctx: &Context<Application>, hold: &Hold, idx: usize) -> Html {
    let link = ctx.link();

    let desc_cb = link.callback(move |e: Event| {
        let value = crate::common::to_string(e);
        PlanMessage::DataChange(PlanChange::HoldDescription(idx, value))
    });

    let rh_cb = link.callback(move |e: Event| {
        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
        PlanMessage::DataChange(PlanChange::HoldRightHand(idx, input.checked()))
    });

    let ibt_cb = link.callback(move |e| on_num(e, idx, PlanChange::HoldInBoundTrack));
    let speed_cb = link.callback(move |e| on_num(e, idx, PlanChange::HoldSpeed));
    let var_cb = link.callback(move |e| on_num(e, idx, PlanChange::HoldVariation));
    let wdir_cb = link.callback(move |e| on_num(e, idx, PlanChange::HoldWindDirection));
    let wspd_cb = link.callback(move |e| on_num(e, idx, PlanChange::HoldWindSpeed));

    let delete = link.callback(move |_| PlanMessage::DataChange(PlanChange::HoldDelete(idx)));

    html!(
        <tr>
            <td>
                <input
                    type="text"
                    value={hold.description.clone()}
                    onchange={desc_cb}
                    style="width:120px"
                />
            </td>
            <td style="text-align:center">
                <input
                    type="checkbox"
                    checked={hold.right_hand}
                    onchange={rh_cb}
                />
            </td>
            <td><input type="number" class="ra" value={hold.in_bound_track.to_string()} onchange={ibt_cb}/></td>
            <td><input type="number" class="ra" value={hold.aircraft_speed.to_string()} onchange={speed_cb}/></td>
            <td><input type="number" class="ra" value={hold.variation.to_string()} onchange={var_cb}/></td>
            <td><input type="number" class="ra" value={hold.wind.angle.to_string()} onchange={wdir_cb}/></td>
            <td><input type="number" class="ra" value={hold.wind.speed.to_string()} onchange={wspd_cb}/></td>
            <td>
                <div class="row-acts" style="opacity:1">
                    <button class="ibtn del" onclick={delete} title="Delete hold">{"×"}</button>
                </div>
            </td>
        </tr>
    )
}

fn on_num(e: Event, idx: usize, f: fn(usize, i64) -> PlanChange) -> PlanMessage {
    PlanMessage::DataChange(f(idx, to_number(e)))
}
