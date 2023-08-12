use crate::application::Application;
use crate::common::to_string;

use crate::messages::{PlanChange, PlanMessage};

use definition::Important;

use web_sys::Event;

use yew::prelude::*;

pub fn important_html(ctx: &Context<Application>, Important { lines }: &Important) -> Html {
    let html: Html = lines
        .iter()
        .enumerate()
        .map(|(idx, s)| create_lines(ctx, idx, s))
        .collect();

    html!(
        <table class="table table-bordered">
          <tbody>
            {html}
          </tbody>
        </table>
    )
}

fn create_lines(ctx: &Context<Application>, idx: usize, line: &Option<String>) -> Html {
    let link = ctx.link();
    let value = or_else(line, "");
    let callback = link.callback(move |e| on_change(e, idx));

    html! {
        <tr>
          <td>
             <input type="text" class="form-control" value={value} onchange={callback}/>
          </td>
        </tr>
    }
}

fn on_change(e: Event, idx: usize) -> PlanMessage {
    let value = to_string(e);
    let change = if value.trim().is_empty() {
        None
    } else {
        Some(value)
    };

    PlanMessage::DataChange(PlanChange::Important(idx, change))
}

fn or_else(value: &Option<String>, default_value: &str) -> String {
    if let Some(value) = value {
        value.to_owned()
    } else {
        default_value.to_owned()
    }
}
