use crate::application::Application;
use crate::common::to_string;

use crate::messages::{PlanChange, PlanMessage};

use definition::Detail;

use web_sys::Event;

use yew::prelude::*;

pub fn details_html(ctx: &Context<Application>, detail: &Detail) -> Html {
    let tail = or_else(&detail.tail, "");
    let call_sign = or_else(&detail.call_sign, "");
    let pic = or_else(&detail.pic, "");

    let link = ctx.link();
    let callback_tail = link.callback(on_change_tail);
    let callback_sign = link.callback(on_change_call_sign);
    let callback_pic = link.callback(on_change_pic);

    html!(
        <table class="table table-bordered">
        <tbody>
        <tr>
          <td><input type="text" class="form-control" value={tail} onchange={callback_tail}/></td>
        </tr>
        <tr>
          <td><input type="text" class="form-control" value={call_sign} onchange={callback_sign}/></td>
        </tr>
        <tr>
          <td><input type="text" class="form-control" value={pic} onchange={callback_pic}/></td>
        </tr>
        </tbody>
        </table>
    )
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

fn or_else(value: &Option<String>, default_value: &str) -> String {
    if let Some(value) = value {
        value.to_owned()
    } else {
        default_value.to_owned()
    }
}
