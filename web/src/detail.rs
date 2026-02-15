use crate::application::Application;
use crate::common::{to_number, to_string};

use crate::messages::{PlanChange, PlanMessage};

use web_sys::Event;

use yew::prelude::*;

pub fn details_html(ctx: &Context<Application>, app: &Application) -> Html {
    let detail = &app.plan.detail;
    let tail = or_else(&detail.tail, "");
    let call_sign = or_else(&detail.call_sign, "");
    let pic = or_else(&detail.pic, "");
    let field1 = or_else(&detail.field1, "");
    let field2 = or_else(&detail.field2, "");
    let field3 = or_else(&detail.field3, "");

    let link = ctx.link();
    let callback_tail = link.callback(on_change_tail);
    let callback_sign = link.callback(on_change_call_sign);
    let callback_pic = link.callback(on_change_pic);
    let callback_field1 = link.callback(on_change_field1);
    let callback_field2 = link.callback(on_change_field2);
    let callback_field3 = link.callback(on_change_field3);

    let registrations = &app.workspace.aircraft_registrations;
    let call_signs = &app.workspace.call_signs;
    let pics = &app.workspace.pics;

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Flight Details"}
                </div>
            </div>
            <div class="panel-body">
                <div style="display:flex; justify-content:space-between;">
                    <div style="display:flex; flex-direction:column; gap:8px; width:130px; flex-shrink:0;">
                        <input class="fg-bare" type="text" value={field1} onchange={callback_field1}/>
                        <input class="fg-bare" type="text" value={field2} onchange={callback_field2}/>
                        <input class="fg-bare" type="text" value={field3} onchange={callback_field3}/>
                    </div>
                    <div style="display:flex; flex-direction:column; gap:8px; flex-shrink:0;">
                        <div style="display:flex; align-items:center; gap:8px;">
                            <label style="font-size:11px; font-weight:600; color:var(--text-dim); white-space:nowrap; width:60px; text-align:right;">{"Tail"}</label>
                            <div style="width:130px;">{ field_input_or_select_bare_ra("detail-tail", &tail, registrations, callback_tail) }</div>
                        </div>
                        <div style="display:flex; align-items:center; gap:8px;">
                            <label style="font-size:11px; font-weight:600; color:var(--text-dim); white-space:nowrap; width:60px; text-align:right;">{"Call Sign"}</label>
                            <div style="width:130px;">{ field_input_or_select_bare_ra("detail-call-sign", &call_sign, call_signs, callback_sign) }</div>
                        </div>
                        <div style="display:flex; align-items:center; gap:8px;">
                            <label style="font-size:11px; font-weight:600; color:var(--text-dim); white-space:nowrap; width:60px; text-align:right;">{"PIC"}</label>
                            <div style="width:130px;">{ field_input_or_select_bare_ra("detail-pic", &pic, pics, callback_pic) }</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

fn field_input_or_select_bare_ra(
    id: &str,
    current: &str,
    options: &[String],
    callback: Callback<Event>,
) -> Html {
    if options.is_empty() {
        html!(<input class="fg-bare" type="text" value={current.to_owned()} onchange={callback}/>)
    } else {
        let list_id = format!("{}-list", id);
        html!(
            <>
                <input class="fg-bare" type="text" value={current.to_owned()} list={list_id.clone()} onchange={callback}/>
                <datalist id={list_id}>
                    { for options.iter().map(|o| html!(<option value={o.clone()}/>)) }
                </datalist>
            </>
        )
    }
}

fn on_change(e: Event, f: fn(Option<String>) -> PlanChange) -> PlanMessage {
    let value = to_string(e);
    let change = if value.trim().is_empty() {
        f(None)
    } else {
        f(Some(value))
    };

    PlanMessage::DataChange(change)
}

fn on_change_tail(e: Event) -> PlanMessage {
    on_change(e, PlanChange::Tail)
}

fn on_change_pic(e: Event) -> PlanMessage {
    on_change(e, PlanChange::PilotInCommand)
}

fn on_change_call_sign(e: Event) -> PlanMessage {
    on_change(e, PlanChange::CallSign)
}

fn on_change_field1(e: Event) -> PlanMessage {
    on_change(e, PlanChange::Field1)
}

fn on_change_field2(e: Event) -> PlanMessage {
    on_change(e, PlanChange::Field2)
}

fn on_change_field3(e: Event) -> PlanMessage {
    on_change(e, PlanChange::Field3)
}

fn or_else(value: &Option<String>, default_value: &str) -> String {
    if let Some(value) = value {
        value.to_owned()
    } else {
        default_value.to_owned()
    }
}

pub fn set_wind_html(ctx: &Context<Application>, app: &Application) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Set Wind"}
                </div>
            </div>
            <div class="panel-body">
                <div style="display:flex; align-items:center; gap:8px;">
                    <label style="font-size:11px; font-weight:600; color:var(--text-dim);">{"DIR"}</label>
                    <input
                        type="number"
                        class="fg-bare ra"
                        style="width:70px;"
                        value={app.wind_all_dir.to_string()}
                        onchange={link.callback(|e: Event| PlanMessage::DataChange(PlanChange::SetWindAllDir(to_number(e))))}
                    />
                    <label style="font-size:11px; font-weight:600; color:var(--text-dim);">{"SPD"}</label>
                    <input
                        type="number"
                        class="fg-bare ra"
                        style="width:70px;"
                        value={app.wind_all_spd.to_string()}
                        onchange={link.callback(|e: Event| PlanMessage::DataChange(PlanChange::SetWindAllSpd(to_number(e))))}
                    />
                    <button
                        class="btn btn-sm"
                        onclick={link.callback(|_| PlanMessage::DataChange(PlanChange::ApplyWindAll))}
                    >
                        {"Apply to All"}
                    </button>
                </div>
            </div>
        </div>
    )
}
