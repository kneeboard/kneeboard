use crate::application::Application;

use crate::leg::legs_html;
use crate::messages::{PlanChange, PlanMessage};
use crate::note::notes_html;

use definition::{Leg, Route};

use crate::icons::{chevron_bar_down, chevron_bar_up, file_earmark_arrow_up, x_circle};

use yew::prelude::*;

pub fn routes_html(ctx: &Context<Application>, app: &Application) -> Html {
    let routes = &app.plan.routes;
    let route_count = routes.len();
    let result: Html = routes
        .iter()
        .enumerate()
        .flat_map(|(idx, r)| {
            let mut elements = vec![];

            // Show insertion dialog above if we're inserting at this index
            if app.inserting_route_at == Some(idx) {
                elements.push(route_insertion_dialog(ctx, app));
            }

            // Show the route
            elements.push(route(ctx, app, idx, r));

            // Show insertion dialog below if we're inserting after this route
            if app.inserting_route_at == Some(idx + 1) && idx + 1 == route_count {
                elements.push(route_insertion_dialog(ctx, app));
            }

            elements.into_iter()
        })
        .collect();

    html!(
        <>
        {result}
        </>
    )
}

fn route(ctx: &Context<Application>, app: &Application, route_idx: usize, route: &Route) -> Html {
    let link = ctx.link();
    let delete_route = link.callback(move |_| on_change_delete(route_idx));
    let insert_route = link.callback(move |_| on_change_insert_route(route_idx));
    let insert_route_below = link.callback(move |_| on_change_insert_route_below(route_idx));
    let save_to_workspace = link.callback(move |_| PlanMessage::SaveRouteToWorkspace(route_idx));

    let legs_html = legs_html(ctx, route_idx, &route.legs);
    let notes_html = notes_html(ctx, route_idx, &route.notes);
    let placeholder = format!("Route {:02}", route_idx + 1);
    let route_meta = leg_name(&route.legs);

    let overwrite_prompt = if app.confirm_overwrite_route.map(|(ri, _)| ri) == Some(route_idx) {
        let saved_name = app
            .confirm_overwrite_route
            .and_then(|(_, wi)| app.workspace.saved_routes.get(wi))
            .map(|s| s.name.clone())
            .unwrap_or_default();
        html!(
            <div style="display:flex; align-items:center; gap:8px; padding:6px 14px; background:var(--bg-secondary); border-top:1px solid var(--border); font-size:13px;">
                <span>{format!("\"{}\" already exists in workspace. Overwrite?", saved_name)}</span>
                <button class="btn btn-sm btn-primary"
                    onclick={link.callback(|_| PlanMessage::ConfirmOverwriteSavedRoute)}>
                    {"Overwrite"}
                </button>
                <button class="btn btn-sm"
                    onclick={link.callback(|_| PlanMessage::CancelOverwriteSavedRoute)}>
                    {"Cancel"}
                </button>
            </div>
        )
    } else {
        html!()
    };

    html!(
        <>
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    <input
                        type="text"
                        class="route-name-input"
                        value={route.name.clone()}
                        placeholder={placeholder}
                        oninput={link.callback(move |e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            PlanMessage::DataChange(PlanChange::RouteName(route_idx, input.value()))
                        })}
                    />
                </div>
                <div style="display: flex; align-items: center; gap: 8px;">
                    <div class="route-meta">{route_meta}</div>
                    <button type="button" class="ibtn" onclick={save_to_workspace} title="Save route to workspace">
                        {file_earmark_arrow_up(18)}
                    </button>
                    <button type="button" class="ibtn" onclick={insert_route} title="Insert route above">
                        {chevron_bar_up(18)}
                    </button>
                    <button type="button" class="ibtn" onclick={insert_route_below} title="Insert route below">
                        {chevron_bar_down(18)}
                    </button>
                    <button type="button" class="ibtn del" onclick={delete_route} title="Delete route">
                        {x_circle(18)}
                    </button>
                </div>
            </div>
            {overwrite_prompt}
            <div class="panel-body" style="padding:8px 14px;">
                {legs_html}
            </div>
        </div>
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Notes"}
                </div>
            </div>
            <div class="panel-body">
                {notes_html}
            </div>
        </div>
        </>
    )
}

#[allow(dead_code)]
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
    PlanMessage::ShowRouteInsertDialog(idx)
}

fn on_change_insert_route_below(idx: usize) -> PlanMessage {
    PlanMessage::ShowRouteInsertBelowDialog(idx)
}

fn route_insertion_dialog(ctx: &Context<Application>, app: &Application) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel" style="border: 2px dashed var(--border); background: var(--bg-secondary);">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Insert New Route"}
                </div>
            </div>
            <div class="panel-body">
                <div class="fg">
                    <label>{"Waypoints (comma-separated)"}</label>
                    <input
                        type="text"
                        placeholder="e.g., EGTF, MAXIT, MID, OCK, EGTF"
                        value={app.insert_waypoints.clone()}
                        autofocus={true}
                        oninput={link.callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            PlanMessage::InsertRouteWaypoints(input.value())
                        })}
                        onkeypress={link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" {
                                PlanMessage::CreateInsertedRoute
                            } else if e.key() == "Escape" {
                                PlanMessage::CancelRouteInsert
                            } else {
                                PlanMessage::SetMessage(String::new())
                            }
                        })}
                    />
                </div>
                <div style="display:flex; gap:8px; margin-top:12px;">
                    <button
                        class="btn btn-primary"
                        onclick={link.callback(|_| PlanMessage::CreateInsertedRoute)}
                    >
                        {"Insert Route"}
                    </button>
                    <button
                        class="btn"
                        onclick={link.callback(|_| PlanMessage::CancelRouteInsert)}
                    >
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    )
}
