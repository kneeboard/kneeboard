use crate::application::Application;
use crate::common::to_files;
use crate::messages::{PlanMessage, WorkspaceChange};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use web_sys::Event;
use yew::prelude::*;

pub fn workspace_page(app: &Application, ctx: &Context<Application>) -> Html {
    html!(
        <div class="main" style="display:block; padding:24px; overflow-y:auto;">
            {file_management_panel(app, ctx)}
            {saved_routes_panel(app, ctx)}
            {aircraft_registrations_panel(app, ctx)}
            {pics_panel(app, ctx)}
            {call_signs_panel(app, ctx)}
            {default_leg_values_panel(app, ctx)}
        </div>
    )
}

fn file_management_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();

    fn on_click_upload(e: Event) -> PlanMessage {
        match to_files(e) {
            Some(files) => {
                if let Some(file) = files.get(0) {
                    PlanMessage::WorkspaceLoadFile(Some(gloo::file::File::from(file)))
                } else {
                    PlanMessage::WorkspaceLoadFile(None)
                }
            }
            None => PlanMessage::WorkspaceLoadFile(None),
        }
    }

    let workspace_json = serde_json::to_string_pretty(&app.workspace).unwrap_or_default();
    let workspace_base64 = STANDARD_NO_PAD.encode(workspace_json.as_bytes());
    let encoded_workspace = format!("data:application/json;base64,{workspace_base64}");

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Workspace File Management"}
                </div>
            </div>
            <div class="panel-body">
                <div style="display:flex; gap:8px; align-items:center;">
                    <div class="image-upload" style="display: inline-block;">
                        <label for="workspaceFileToUpload" title="Load workspace" class="btn" style="cursor:pointer;">
                            {"Load Workspace"}
                        </label>
                        <input
                            type="file"
                            style="display:none"
                            name="workspaceFileToUpload"
                            id="workspaceFileToUpload"
                            accept=".json"
                            multiple={false}
                            value=""
                            onchange={link.callback(on_click_upload)}/>
                    </div>
                    <a download="workspace.json" title="Save workspace" href={encoded_workspace}>
                        <button class="btn">{"Export Workspace"}</button>
                    </a>
                </div>
                <div style="margin-top:8px; font-size:12px; color:var(--text-dim);">
                    {"Workspace is automatically saved to browser storage. Use Export/Load for backups or device transfer."}
                </div>
            </div>
        </div>
    )
}

fn aircraft_registrations_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel" style="margin-top:24px;">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Aircraft Registrations"}
                </div>
            </div>
            <div class="panel-body">
                <table class="table">
                    <thead>
                        <tr>
                            <th>{"Registration"}</th>
                            <th style="width:100px;">{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {app.workspace.aircraft_registrations.iter().enumerate().map(|(idx, reg)| {
                            html!(
                                <tr key={idx}>
                                    <td>
                                        <input
                                            type="text"
                                            value={reg.clone()}
                                            oninput={link.callback(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                PlanMessage::WorkspaceChange(WorkspaceChange::RegistrationUpdate(idx, input.value()))
                                            })}
                                        />
                                    </td>
                                    <td>
                                        <button
                                            class="btn btn-sm"
                                            onclick={link.callback(move |_| {
                                                PlanMessage::WorkspaceChange(WorkspaceChange::RegistrationDelete(idx))
                                            })}
                                        >
                                            {"Delete"}
                                        </button>
                                    </td>
                                </tr>
                            )
                        }).collect::<Html>()}
                    </tbody>
                </table>
                <button
                    class="btn"
                    onclick={link.callback(|_| {
                        PlanMessage::WorkspaceChange(WorkspaceChange::RegistrationAdd(String::new()))
                    })}
                >
                    {"Add Registration"}
                </button>
            </div>
        </div>
    )
}

fn pics_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel" style="margin-top:24px;">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Pilots In Command (PICs)"}
                </div>
            </div>
            <div class="panel-body">
                <table class="table">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th style="width:100px;">{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {app.workspace.pics.iter().enumerate().map(|(idx, pic)| {
                            html!(
                                <tr key={idx}>
                                    <td>
                                        <input
                                            type="text"
                                            value={pic.clone()}
                                            oninput={link.callback(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                PlanMessage::WorkspaceChange(WorkspaceChange::PicUpdate(idx, input.value()))
                                            })}
                                        />
                                    </td>
                                    <td>
                                        <button
                                            class="btn btn-sm"
                                            onclick={link.callback(move |_| {
                                                PlanMessage::WorkspaceChange(WorkspaceChange::PicDelete(idx))
                                            })}
                                        >
                                            {"Delete"}
                                        </button>
                                    </td>
                                </tr>
                            )
                        }).collect::<Html>()}
                    </tbody>
                </table>
                <button
                    class="btn"
                    onclick={link.callback(|_| {
                        PlanMessage::WorkspaceChange(WorkspaceChange::PicAdd(String::new()))
                    })}
                >
                    {"Add PIC"}
                </button>
            </div>
        </div>
    )
}

fn call_signs_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel" style="margin-top:24px;">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Call Signs"}
                </div>
            </div>
            <div class="panel-body">
                <table class="table">
                    <thead>
                        <tr>
                            <th>{"Call Sign"}</th>
                            <th style="width:100px;">{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {app.workspace.call_signs.iter().enumerate().map(|(idx, cs)| {
                            html!(
                                <tr key={idx}>
                                    <td>
                                        <input
                                            type="text"
                                            value={cs.clone()}
                                            oninput={link.callback(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                PlanMessage::WorkspaceChange(WorkspaceChange::CallSignUpdate(idx, input.value()))
                                            })}
                                        />
                                    </td>
                                    <td>
                                        <button
                                            class="btn btn-sm"
                                            onclick={link.callback(move |_| {
                                                PlanMessage::WorkspaceChange(WorkspaceChange::CallSignDelete(idx))
                                            })}
                                        >
                                            {"Delete"}
                                        </button>
                                    </td>
                                </tr>
                            )
                        }).collect::<Html>()}
                    </tbody>
                </table>
                <button
                    class="btn"
                    onclick={link.callback(|_| {
                        PlanMessage::WorkspaceChange(WorkspaceChange::CallSignAdd(String::new()))
                    })}
                >
                    {"Add Call Sign"}
                </button>
            </div>
        </div>
    )
}

fn default_leg_values_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();
    let defaults = &app.workspace.default_leg_values;

    html!(
        <div class="panel" style="margin-top:24px;">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Default Leg Values"}
                </div>
            </div>
            <div class="panel-body">
                <div style="display:grid; grid-template-columns:1fr 1fr; gap:16px;">
                    <div class="fg">
                        <label>{"Speed (kts)"}</label>
                        <input
                            type="number"
                            value={defaults.speed.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultSpeed(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Course (°)"}</label>
                        <input
                            type="number"
                            value={defaults.course.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultCourse(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Distance (nm)"}</label>
                        <input
                            type="number"
                            value={defaults.distance.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultDistance(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Variation (°)"}</label>
                        <input
                            type="number"
                            value={defaults.variation.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultVariation(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Wind Direction (°)"}</label>
                        <input
                            type="number"
                            value={defaults.wind_direction.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultWindDirection(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Wind Speed (kts)"}</label>
                        <input
                            type="number"
                            value={defaults.wind_speed.to_string()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse().unwrap_or(0);
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultWindSpeed(val))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Safe Altitude"}</label>
                        <input
                            type="text"
                            value={defaults.safe.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultSafe(input.value()))
                            })}
                        />
                    </div>

                    <div class="fg">
                        <label>{"Planned Altitude"}</label>
                        <input
                            type="text"
                            value={defaults.planned.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                PlanMessage::WorkspaceChange(WorkspaceChange::DefaultPlanned(input.value()))
                            })}
                        />
                    </div>
                </div>
            </div>
        </div>
    )
}

fn saved_routes_panel(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();

    html!(
        <div class="panel" style="margin-top:24px; margin-bottom:24px;">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Saved Routes"}
                </div>
            </div>
            <div class="panel-body">
                if app.workspace.saved_routes.is_empty() {
                    <div style="text-align:center; padding:24px; color:var(--text-dim);">
                        {"No saved routes. Add one below."}
                    </div>
                } else {
                    <table class="table">
                        <thead>
                            <tr>
                                <th>{"Name"}</th>
                                <th>{"Waypoints (CSV)"}</th>
                                <th>{"Legs"}</th>
                                <th style="width:150px;">{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {app.workspace.saved_routes.iter().enumerate().map(|(idx, route)| {
                                html!(
                                    <tr key={idx}>
                                        <td>
                                            <input
                                                type="text"
                                                value={route.name.clone()}
                                                placeholder="Route name"
                                                oninput={link.callback(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    PlanMessage::WorkspaceChange(WorkspaceChange::SavedRouteName(idx, input.value()))
                                                })}
                                            />
                                        </td>
                                        <td>
                                            <input
                                                type="text"
                                                value={route.waypoints.clone()}
                                                placeholder="e.g., EGTF, MAXIT, MID"
                                                oninput={link.callback(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    PlanMessage::WorkspaceChange(WorkspaceChange::SavedRouteWaypoints(idx, input.value()))
                                                })}
                                            />
                                        </td>
                                        <td>{route.legs.len()}</td>
                                        <td>
                                            <div style="display:flex; gap:4px;">
                                                <button
                                                    class="btn btn-sm"
                                                    onclick={link.callback(move |_| {
                                                        PlanMessage::WorkspaceChange(WorkspaceChange::SavedRouteLoadToPlan(idx))
                                                    })}
                                                >
                                                    {"Load"}
                                                </button>
                                                <button
                                                    class="btn btn-sm"
                                                    onclick={link.callback(move |_| {
                                                        PlanMessage::WorkspaceChange(WorkspaceChange::SavedRouteDelete(idx))
                                                    })}
                                                >
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                )
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                }
            </div>
        </div>
    )
}
