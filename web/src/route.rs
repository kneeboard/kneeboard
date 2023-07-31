use crate::application::Application;

use crate::leg::legs_html;
use crate::messages::{PlanChange, PlanMessage};
use crate::note::notes_html;

use core::definition::{Leg, Route};

use crate::icons::{chevron_bar_up, plus_circle, x_circle};

use yew::prelude::*;

pub fn routes_html(ctx: &Context<Application>, routes: &[Route]) -> Html {
    let link = ctx.link();
    let append_route = link.callback(move |_| on_change_append_route());

    if routes.is_empty() {
        html!(
            <button type="button" class="btn btn-link" onclick={append_route}>
                {plus_circle(32)}
            </button>
        )
    } else {
        let result: Html = routes
            .iter()
            .enumerate()
            .map(|(idx, r)| route(ctx, idx, r))
            .collect();

        html!(
            <>
            {result}
            <div class="ms-4">
              <button type="button" class="btn btn-link" onclick={append_route}>
                {plus_circle(32)}
              </button>
            </div>
            </>
        )
    }
}

fn route(ctx: &Context<Application>, route_idx: usize, route: &Route) -> Html {
    let link = ctx.link();
    let delete_route = link.callback(move |_| on_change_delete(route_idx));
    let insert_route = link.callback(move |_| on_change_insert_route(route_idx));

    let legs_html = legs_html(ctx, route_idx, &route.legs);
    let notes_html = notes_html(ctx, route_idx, &route.notes);
    let title = leg_name(&route.legs);
    html!(
        <div class="ms-4">

          <div class="panel panel-success,ms-2">
            <div class="panel-heading">
              <table>
                <tr>
                  <td style="padding: 0px" valign="middle">
                    {title}
                  </td>
                  <td style="padding: 0px" valign="middle">
                    <button type="button" class="btn btn-link" onclick={delete_route}>
                      {x_circle(26)}
                    </button>
                  </td>
                  <td style="padding: 0px" valign="middle">
                    <button type="button" class="btn btn-link" onclick={insert_route}>
                      {chevron_bar_up(32)}
                    </button>
                  </td>
                </tr>
              </table>
            </div>
          </div>
          <div class="panel-body">
            {legs_html}
            <p>{"Notes:"}</p>
            {notes_html}
          </div>
        </div>
    )
}

fn on_change_append_route() -> PlanMessage {
    PlanMessage::DataChange(PlanChange::RouteAppend)
}

fn leg_name(legs: &[Leg]) -> String {
    match legs {
        [] => "-- to --".to_owned(),
        [leg] => format!("{} to {}", leg.from, leg.to),
        [first, .., last] => format!("{} to {}", first.from, last.to),
    }
}

fn on_change_delete(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::RouteDelete(idx))
}

fn on_change_insert_route(idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::RouteInsert(idx))
}
