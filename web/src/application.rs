use crate::common::to_files;
use crate::detail::{details_html, set_wind_html};
use crate::diversion::diversion_html;
use crate::hold::hold_html;
use crate::messages::{AppPage, LoadedFileDetails, PlanChange, PlanMessage, ProfileChange};
use crate::route::routes_html;
use crate::workspace_storage;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use common::{
    create_template_diversion, create_template_hold, create_template_leg_with_from,
    create_template_route, KneeboardError,
};
use core::planner::create_planning;
use definition::{
    Diversion, FontType, Hold, Leg, Plan, ProfileConfig, Route, SavedHold, SavedRoute, Velocity,
};
use gloo_console::__macro::JsValue;

use gloo::file::callbacks::read_as_bytes;
use gloo::file::{callbacks::FileReader, File};

use crate::icons::{info_circle, layout_text_sidebar, sun_moon};
use std::collections::HashMap;
use web_sys::{Event, FileList};
use yew::{html::Scope, prelude::*};

impl Component for Application {
    type Message = PlanMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let profile = workspace_storage::load_profile_from_local_storage().unwrap_or_default();
        let mut app = Application {
            profile,
            current_page: AppPage::FlightPlanning,
            ..Default::default()
        };
        app.update_data();
        app.toggle_layout();
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.clear_message();
        match msg {
            PlanMessage::Files(Some(file)) => submit_load(self, file, ctx.link().clone()),
            PlanMessage::Files(None) => self.message = Some("No file loaded".to_string()),
            PlanMessage::Loaded(details) => update_plan(self, details),
            PlanMessage::DataChange(change) => handle_plan_change(self, change),
            PlanMessage::LayoutToggle => self.toggle_layout(),
            PlanMessage::ThemeToggle => toggle_theme(),
            PlanMessage::SetMessage(msg) => self.message = Some(msg),

            // Navigation
            PlanMessage::NavigateTo(page) => self.current_page = page,

            // Initial Route Creation
            PlanMessage::InitialWaypointsInput(value) => self.waypoint_input = value,
            PlanMessage::CreateInitialRoute => handle_create_initial_route(self),
            PlanMessage::SelectSavedRoute(idx) => self.selected_saved_route = idx,

            // Save/Load routes to/from workspace
            PlanMessage::SaveRouteToProfile(idx) => handle_save_route_to_workspace(self, idx),
            PlanMessage::ConfirmOverwriteSavedRoute => handle_confirm_overwrite(self),
            PlanMessage::CancelOverwriteSavedRoute => self.confirm_overwrite_route = None,

            // Route Insertion with Waypoints
            PlanMessage::ShowRouteInsertDialog(idx) => {
                self.inserting_route_at = Some(idx);
                self.insert_waypoints = String::new();
            }
            PlanMessage::ShowRouteInsertBelowDialog(idx) => {
                self.inserting_route_at = Some(idx + 1);
                self.insert_waypoints = String::new();
            }
            PlanMessage::InsertRouteWaypoints(value) => self.insert_waypoints = value,
            PlanMessage::CreateInsertedRoute => handle_create_inserted_route(self),
            PlanMessage::CancelRouteInsert => {
                self.inserting_route_at = None;
                self.insert_waypoints = String::new();
            }

            // Workspace Management
            PlanMessage::ProfileLoadFile(Some(file)) => {
                submit_profile_load(self, file, ctx.link().clone())
            }
            PlanMessage::ProfileLoadFile(None) => {
                self.message = Some("No profile file loaded".to_string())
            }
            PlanMessage::ProfileLoaded(details) => update_profile(self, details),
            PlanMessage::ProfileChange(change) => handle_profile_change(self, change),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let topbar_html = topbar_html(self, ctx);
        let content_html = match self.current_page {
            AppPage::FlightPlanning => flight_planning_page(self, ctx),
            AppPage::Profile => crate::workspace::profile_page(self, ctx),
            AppPage::About => about_page(ctx),
        };

        html!(
            <>
                {topbar_html}

                if let Some(msg) = &self.message {
                    <div class="alert alert-danger" role="alert">
                        {msg}
                    </div>
                }

                {content_html}
            </>
        )
    }
}

fn topbar_html(app: &Application, ctx: &Context<Application>) -> Html {
    fn on_click_upload(e: Event) -> PlanMessage {
        upload_files(to_files(e))
    }

    fn on_click_toggle_layout(_: MouseEvent) -> PlanMessage {
        PlanMessage::LayoutToggle
    }

    let json_base64 = STANDARD_NO_PAD.encode(&app.json);
    let encoded_json = format!("data:application/json;base64,{json_base64}");

    html!(
        <div class="topbar">
            <div class="topbar-left">
                <div class="topbar-brand">{"Kneeboard"}</div>
                <div class="topbar-sub">{"Flight Planning System"}</div>
                <div style="display:flex; gap:4px; margin-left:24px; border-left:1px solid var(--border); padding-left:24px;">
                    <button
                        class={if app.current_page == AppPage::FlightPlanning {"btn btn-primary"} else {"btn"}}
                        onclick={ctx.link().callback(|_| PlanMessage::NavigateTo(AppPage::FlightPlanning))}
                    >
                        {"Planning"}
                    </button>
                    <button
                        class={if app.current_page == AppPage::Profile {"btn btn-primary"} else {"btn"}}
                        onclick={ctx.link().callback(|_| PlanMessage::NavigateTo(AppPage::Profile))}
                    >
                        {"Profile"}
                    </button>
                </div>
            </div>
            <div class="topbar-actions">
                <div class="image-upload" style="display: inline-block;">
                    <label for="fileToUpload" title="Load notes" class="btn" style="cursor:pointer;">
                        {"Load"}
                    </label>
                    <input
                        type="file"
                        style="display:none"
                        name="fileToUpload"
                        id="fileToUpload"
                        multiple={false}
                        value=""
                        onchange={ctx.link().callback(on_click_upload)}/>
                </div>
                <a download="kneeboard-notes.json" title="Save notes" href={encoded_json}>
                    <button class="btn">{"Export"}</button>
                </a>
                <button class="btn btn-link" onclick={ctx.link().callback(on_click_toggle_layout)} title="Toggle Layout">
                    {layout_text_sidebar(24)}
                </button>
                <button class="btn btn-link" onclick={ctx.link().callback(|_| PlanMessage::ThemeToggle)} title="Toggle Theme">
                    {sun_moon(20)}
                </button>
                <button class="btn btn-link" onclick={ctx.link().callback(|_| PlanMessage::NavigateTo(AppPage::About))} title="About">
                    {info_circle(20)}
                </button>
            </div>
        </div>
    )
}

fn toggle_theme() {
    let _ = js_sys::eval("window.__toggleTheme()");
}

fn pdf_preview_html(app: &Application) -> Html {
    if app.pdf.is_empty() {
        html!(
            <div style="text-align:center;">
                <div style="font-size:36px;opacity:0.2;margin-bottom:8px;">{"✈"}</div>
                <div style="font-size:13px;">{"A5 Kneeboard PDF"}</div>
                <div style="font-size:11px;margin-top:4px;color:var(--text-dim);">{"Real-time render"}</div>
            </div>
        )
    } else {
        let pdf_base64 = STANDARD_NO_PAD.encode(&app.pdf);
        let embed = format!("data:application/pdf;base64,{pdf_base64}");

        html!(
            <embed title="kneeboard-notes.pdf" type="application/pdf" width="100%" height="100%" src={embed}/>
        )
    }
}

fn about_page(ctx: &Context<Application>) -> Html {
    html!(
        <div class="main" style="display:block; padding:24px; overflow-y:auto;">
            <div class="panel" style="max-width:700px;">
                <div class="panel-head">
                    <div class="panel-title">
                        <span class="marker"></span>
                        {"About Kneeboard Notes"}
                    </div>
                </div>
                <div class="panel-body" style="line-height:1.8;">
                    <div class="alert alert-warning" style="margin-bottom:20px;">
                        <strong>{"⚠ Work in Progress"}</strong>
                        {" — This is a personal project and is still under active development. Features may be incomplete or change without notice."}
                    </div>

                    <div style="margin-bottom:20px;">
                        <p>
                            {"Kneeboard Notes provides a way to create notes for A5 aviation kneeboards. The format of the generated notes is primarily relevant to PPL(A) in the UK."}
                        </p>
                        <p style="margin-top:12px;">
                            {"Notes are generated as a PDF document for printing and can be saved as JSON."}
                        </p>
                    </div>

                    <div class="panel" style="margin-bottom:20px; background:var(--bg-deep);">
                        <div class="panel-body">
                            <strong style="color:var(--danger);">{"Disclaimer"}</strong>
                            <p style="margin-top:8px;">
                                {"Do not use the notes generated by this software — they are for illustrative purposes only."}
                            </p>
                            <p style="margin-top:8px;">
                                {"Any reliance you place on this software and/or the generated notes is "}
                                <strong>{"strictly at your own risk."}</strong>
                            </p>
                        </div>
                    </div>

                    <div style="margin-bottom:20px;">
                        <p>
                            {"This project is open source and hosted on GitHub: "}
                            <a href="https://github.com/kneeboard/kneeboard" target="_blank" style="color:var(--accent);">
                                {"github.com/kneeboard/kneeboard"}
                            </a>
                        </p>
                        <p style="margin-top:8px; font-size:13px; color:var(--text-dim);">
                            {"Licensed under the Apache License, Version 2.0."}
                        </p>
                    </div>

                    <button
                        class="btn btn-primary"
                        onclick={ctx.link().callback(|_| PlanMessage::NavigateTo(AppPage::FlightPlanning))}
                    >
                        {"Back to Planning"}
                    </button>
                </div>
            </div>
        </div>
    )
}

fn flight_planning_page(app: &Application, ctx: &Context<Application>) -> Html {
    if app.plan.routes.is_empty() {
        // Show initial waypoint input dialog when no routes exist
        initial_waypoint_dialog(app, ctx)
    } else {
        // Show normal planning view
        let form_html = main_form(app, ctx);
        let pdf_html = pdf_preview_html(app);

        html!(
            <div class="main">
                <div class={if app.layout_vertical { "form-area" } else { "form-area form-area--full" }}>
                    {form_html}
                </div>
                if app.layout_vertical {
                    <div class="preview-area">
                        <div class="preview-label">{"Output Preview"}</div>
                        <div class="preview-content">
                            {pdf_html}
                        </div>
                    </div>
                }
            </div>
        )
    }
}

fn initial_waypoint_dialog(app: &Application, ctx: &Context<Application>) -> Html {
    let link = ctx.link();
    let sel = app.selected_saved_route;

    fn on_workspace_upload(e: Event) -> PlanMessage {
        match crate::common::to_files(e) {
            Some(files) => match files.get(0) {
                Some(file) => PlanMessage::ProfileLoadFile(Some(File::from(file))),
                None => PlanMessage::ProfileLoadFile(None),
            },
            None => PlanMessage::ProfileLoadFile(None),
        }
    }

    html!(
        <div class="main" style="display:flex; align-items:center; justify-content:center;">
            <div class="panel" style="max-width:760px; width:100%; margin:24px;">
                <div class="panel-head">
                    <div class="panel-title">
                        <span class="marker"></span>
                        {"Get Started"}
                    </div>
                </div>
                <div class="panel-body" style="padding:0; display:flex;">

                    <div style="flex:1; padding:24px;">
                        <div style="font-size:13px; font-weight:700; color:var(--text); margin-bottom:8px;">{"Create a Route"}</div>
                        <p style="margin-bottom:16px; color:var(--text-dim); font-size:13px; line-height:1.6;">
                            {"Enter waypoints as a comma-separated list. Legs will be generated between consecutive waypoints using your workspace defaults."}
                        </p>
                        <div class="fg">
                            <label>{"Waypoints (comma-separated)"}</label>
                            <input
                                type="text"
                                placeholder="e.g., EGTF, MAXIT, MID, OCK, EGTF"
                                value={app.waypoint_input.clone()}
                                oninput={link.callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    PlanMessage::InitialWaypointsInput(input.value())
                                })}
                                onkeypress={link.callback(|e: KeyboardEvent| {
                                    if e.key() == "Enter" {
                                        PlanMessage::CreateInitialRoute
                                    } else {
                                        PlanMessage::SetMessage(String::new())
                                    }
                                })}
                            />
                        </div>
                        <div style="margin-top:16px;">
                            <button
                                class="btn btn-primary"
                                onclick={link.callback(|_| PlanMessage::CreateInitialRoute)}
                            >
                                {"Create Route"}
                            </button>
                        </div>
                        if !app.profile.saved_routes.is_empty() {
                            <div style="margin-top:20px; border-top:1px solid var(--border); padding-top:16px;">
                                <div style="font-size:12px; color:var(--text-dim); margin-bottom:8px;">{"Or load a saved route:"}</div>
                                <div style="display:flex; gap:8px; align-items:stretch; min-width:0; width:100%;">
                                    <select
                                        class="fg-bare"
                                        style="flex:1; min-width:0;"
                                        onchange={link.callback(|e: Event| {
                                            let select: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            PlanMessage::SelectSavedRoute(select.value().parse::<usize>().unwrap_or(0))
                                        })}
                                    >
                                        {app.profile.saved_routes.iter().enumerate().map(|(idx, route)| {
                                            let name = if route.name.is_empty() {
                                                format!("Route {}", idx + 1)
                                            } else {
                                                route.name.clone()
                                            };
                                            html!(<option key={idx} value={idx.to_string()}>{name}</option>)
                                        }).collect::<Html>()}
                                    </select>
                                    <button
                                        class="btn btn-primary"
                                        style="flex-shrink:0;"
                                        onclick={link.callback(move |_| {
                                            PlanMessage::ProfileChange(ProfileChange::SavedRouteLoadToPlan(sel))
                                        })}
                                    >
                                        {crate::icons::file_earmark_arrow_down(14)}
                                        {" Load"}
                                    </button>
                                </div>
                            </div>
                        }
                    </div>

                    <div style="width:1px; background:var(--border); margin:24px 0;"></div>

                    <div style="flex:0 0 260px; padding:24px;">
                        <div style="font-size:13px; font-weight:700; color:var(--text); margin-bottom:8px;">{"Load Profile"}</div>
                        <p style="color:var(--text-dim); font-size:13px; line-height:1.6; margin-bottom:16px;">
                            {"A profile stores your aircraft registrations, pilot names, call signs, default leg values, and saved routes — load one before creating a route to pre-fill your plan."}
                        </p>
                        <div class="image-upload" style="display:inline-block;">
                            <label for="initialProfileFile" class="btn" style="cursor:pointer; display:flex; align-items:center; gap:6px;">
                                {crate::icons::file_earmark_arrow_down(14)}
                                {"Load Profile"}
                            </label>
                            <input
                                type="file"
                                style="display:none"
                                id="initialProfileFile"
                                accept=".json"
                                multiple={false}
                                value=""
                                onchange={link.callback(on_workspace_upload)}
                            />
                        </div>
                        <div style="margin-top:12px;">
                            <button
                                class="btn-link"
                                style="font-size:12px; color:var(--text-dim); background:none; border:none; cursor:pointer; padding:0;"
                                onclick={link.callback(|_| PlanMessage::NavigateTo(AppPage::Profile))}
                            >
                                {"Configure profile settings →"}
                            </button>
                        </div>
                    </div>

                </div>
            </div>
        </div>
    )
}

fn main_form(app: &Application, ctx: &Context<Application>) -> Html {
    let details_html = details_html(ctx, app);
    let set_wind_html = set_wind_html(ctx, app);
    let routes_html = routes_html(ctx, app);
    let deviation_html = diversion_html(ctx, &app.plan.diversions);
    let holds_html = hold_html(ctx, &app.plan.holds);
    let saved_routes_html = plan_saved_routes_html(app, ctx);

    html!(
        <>
            {details_html}
            {set_wind_html}
            {routes_html}
            {saved_routes_html}
            {deviation_html}
            {holds_html}
        </>
    )
}

fn plan_saved_routes_html(app: &Application, ctx: &Context<Application>) -> Html {
    if app.profile.saved_routes.is_empty() {
        return html!();
    }

    let link = ctx.link();
    let sel = app.selected_saved_route;

    html!(
        <div class="panel">
            <div class="panel-head">
                <div class="panel-title">
                    <span class="marker"></span>
                    {"Saved Routes"}
                </div>
            </div>
            <div class="panel-body">
                <div style="display:flex; gap:8px; align-items:stretch; min-width:0; width:100%;">
                    <select
                        class="fg-bare"
                        style="flex:1; min-width:0;"
                        onchange={link.callback(|e: Event| {
                            let select: web_sys::HtmlInputElement = e.target_unchecked_into();
                            PlanMessage::SelectSavedRoute(select.value().parse::<usize>().unwrap_or(0))
                        })}
                    >
                        {app.profile.saved_routes.iter().enumerate().map(|(idx, route)| {
                            let name = if route.name.is_empty() {
                                format!("Route {}", idx + 1)
                            } else {
                                route.name.clone()
                            };
                            html!(<option key={idx} value={idx.to_string()}>{name}</option>)
                        }).collect::<Html>()}
                    </select>
                    <button
                        class="btn"
                        style="flex-shrink:0;"
                        onclick={link.callback(move |_| {
                            PlanMessage::ProfileChange(ProfileChange::SavedRouteLoadToPlan(sel))
                        })}
                        title="Load into plan"
                    >
                        {crate::icons::file_earmark_arrow_down(14)}
                        {" "}{"Load"}
                    </button>
                </div>
            </div>
        </div>
    )
}

fn handle_plan_change(app: &mut Application, change: PlanChange) {
    match change {
        PlanChange::Tail(tail) => app.plan.detail.tail = tail,
        PlanChange::PilotInCommand(pic) => app.plan.detail.pic = pic,
        PlanChange::CallSign(call_sign) => app.plan.detail.call_sign = call_sign,
        PlanChange::Field1(v) => app.plan.detail.field1 = v,
        PlanChange::Field2(v) => app.plan.detail.field2 = v,
        PlanChange::Field3(v) => app.plan.detail.field3 = v,

        PlanChange::LegFrom(idx, value) => app.get_leg(idx).from = value,
        PlanChange::LegTo(idx, value) => app.get_leg(idx).to = value,
        PlanChange::LegSafe(idx, value) => app.get_leg(idx).safe = value,
        PlanChange::LegPlanned(idx, value) => app.get_leg(idx).planned = value,
        PlanChange::LegSpeed(idx, value) => app.get_leg(idx).speed = value,
        PlanChange::LegCourse(idx, value) => app.get_leg(idx).course = value,
        PlanChange::LegDistance(idx, value) => app.get_leg(idx).distance = value,
        PlanChange::LegVariation(idx, value) => app.get_leg(idx).variation = value,
        PlanChange::LegWindDirection(idx, value) => app.get_leg(idx).wind_direction = value,
        PlanChange::LegWindSpeed(idx, value) => app.get_leg(idx).wind_speed = value,
        PlanChange::LegAppend(route_idx) => {
            let from = app.get_last_leg_to(route_idx);
            app.append_leg(route_idx, create_template_leg_with_from(from))
        }
        PlanChange::LegDelete(idx) => app.delete_leg(idx),
        PlanChange::LegInsert(idx) => {
            let from = app.get_previous_leg_to(idx);
            app.insert_leg(idx, create_template_leg_with_from(from))
        }

        PlanChange::RouteName(idx, name) => {
            if let Some(route) = app.plan.routes.get_mut(idx) {
                route.name = name;
            }
        }
        PlanChange::RouteAppend => app.append_route(create_template_route()),
        PlanChange::RouteInsert(idx) => app.insert_route(idx, create_template_route()),
        PlanChange::RouteDelete(idx) => app.delete_route(idx),

        PlanChange::NoteBold(idx) => app.set_note_font_bold(idx),
        PlanChange::NoteItalics(idx) => app.set_note_font_italics(idx),
        PlanChange::NoteNormal(idx) => app.set_note_font_normal(idx),
        PlanChange::NoteUpdate(idx, note) => app.update_note(idx, note),
        PlanChange::NoteAppend(route_idx, note) => app.append_note(route_idx, note),
        PlanChange::NoteInsert(idx, note) => app.insert_note(idx, note),
        PlanChange::NoteDelete(idx) => app.delete_note(idx),

        PlanChange::DiversionAppend => app.append_diversion(create_template_diversion()),
        PlanChange::DiversionInsert(idx) => app.insert_diversion(idx, create_template_diversion()),
        PlanChange::DiversionDelete(idx) => app.delete_diversion(idx),
        PlanChange::DiversionSpeed(idx, value) => app.get_diversion(idx).aircraft_speed = value,
        PlanChange::DiversionVariation(idx, value) => app.get_diversion(idx).variation = value,
        PlanChange::DiversionWindDirection(idx, value) => app.get_diversion(idx).wind.angle = value,
        PlanChange::DiversionWindSpeed(idx, value) => app.get_diversion(idx).wind.speed = value,

        PlanChange::HoldAppend => app.plan.holds.push(create_template_hold()),
        PlanChange::HoldDelete(idx) => {
            app.plan.holds.remove(idx);
        }
        PlanChange::HoldDescription(idx, value) => app.get_hold(idx).description = value,
        PlanChange::HoldRightHand(idx, value) => app.get_hold(idx).right_hand = value,
        PlanChange::HoldInBoundTrack(idx, value) => app.get_hold(idx).in_bound_track = value,
        PlanChange::HoldSpeed(idx, value) => app.get_hold(idx).aircraft_speed = value,
        PlanChange::HoldVariation(idx, value) => app.get_hold(idx).variation = value,
        PlanChange::HoldWindDirection(idx, value) => app.get_hold(idx).wind.angle = value,
        PlanChange::HoldWindSpeed(idx, value) => app.get_hold(idx).wind.speed = value,

        PlanChange::RouteFillSafe(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.safe = value.clone();
            }
        }
        PlanChange::RouteFillPlanned(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.planned = value.clone();
            }
        }
        PlanChange::RouteFillSpeed(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.speed = value;
            }
        }
        PlanChange::RouteFillCourse(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.course = value;
            }
        }
        PlanChange::RouteFillDistance(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.distance = value;
            }
        }
        PlanChange::RouteFillVariation(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.variation = value;
            }
        }
        PlanChange::RouteFillWindDirection(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.wind_direction = value;
            }
        }
        PlanChange::RouteFillWindSpeed(route_idx, value) => {
            for leg in &mut app.plan.routes[route_idx].legs {
                leg.wind_speed = value;
            }
        }
        PlanChange::SetWindAllDir(value) => {
            app.wind_all_dir = value;
        }
        PlanChange::SetWindAllSpd(value) => {
            app.wind_all_spd = value;
        }
        PlanChange::ApplyWindAll => {
            let dir = app.wind_all_dir;
            let spd = app.wind_all_spd;
            for route in &mut app.plan.routes {
                for leg in &mut route.legs {
                    leg.wind_direction = dir;
                    leg.wind_speed = spd;
                }
            }
            for hold in &mut app.plan.holds {
                hold.wind.angle = dir;
                hold.wind.speed = spd;
            }
            for diversion in &mut app.plan.diversions {
                diversion.wind.angle = dir;
                diversion.wind.speed = spd;
            }
        }
    }

    app.update_data();
}

fn upload_files(files: Option<FileList>) -> PlanMessage {
    if let Some(files) = files {
        let iter_option = match js_sys::try_iter(&files) {
            Ok(value) => value,
            Err(err) => {
                return PlanMessage::SetMessage(err.as_string().unwrap_or(format!("{:?}", err)))
            }
        };
        if let Some(mut iter) = iter_option {
            fn to_message(result: Result<JsValue, JsValue>) -> PlanMessage {
                match result {
                    Ok(value) => PlanMessage::Files(Some(File::from(web_sys::File::from(value)))),
                    Err(err) => {
                        PlanMessage::SetMessage(err.as_string().unwrap_or(format!("{:?}", err)))
                    }
                }
            }

            iter.next().map_or(PlanMessage::Files(None), to_message)
        } else {
            PlanMessage::Files(None)
        }
    } else {
        PlanMessage::Files(None)
    }
}

#[warn(unused_must_use)]
fn update_plan(app: &mut Application, details: LoadedFileDetails) {
    match decode_plan(app, details) {
        Ok(mut plan) => {
            if !plan.aircraft_registrations.is_empty() {
                app.profile.aircraft_registrations =
                    plan.aircraft_registrations.drain(..).collect();
            }
            if !plan.pics.is_empty() {
                app.profile.pics = plan.pics.drain(..).collect();
            }
            if !plan.call_signs.is_empty() {
                app.profile.call_signs = plan.call_signs.drain(..).collect();
            }
            app.plan = plan;
            app.update_data();
        }
        Err(err) => app.message = Some(err.to_err_string()),
    }
}

fn decode_plan(app: &mut Application, details: LoadedFileDetails) -> Result<Plan, KneeboardError> {
    let LoadedFileDetails {
        id,
        file_name,
        data,
    } = details;
    app.readers.remove(&id);

    let data = if let Ok(data) = data {
        data
    } else {
        return Err(KneeboardError::String(
            "Failed to read loaded file".to_owned(),
        ));
    };

    let is_json = {
        let file_name = file_name.to_lowercase();
        file_name.ends_with(".json") || file_name.ends_with(".jsn")
    };

    if is_json {
        Ok(serde_json::from_reader(&data[..])?)
    } else {
        Err(KneeboardError::String(
            "Unsupported file type (expect .json)".to_owned(),
        ))
    }
}

fn submit_load(app: &mut Application, file: File, link: Scope<Application>) {
    let file_name = file.name();

    let id = app.get_next_id();
    let task = {
        read_as_bytes(&file, move |data| {
            let details = LoadedFileDetails {
                id,
                file_name,
                data,
            };
            link.send_message(PlanMessage::Loaded(details))
        })
    };
    app.readers.insert(id, task);
}

#[derive(Default)]
pub struct Application {
    pub plan: Plan,
    pub message: Option<String>,
    pub readers: HashMap<usize, FileReader>,
    pub pdf: Vec<u8>,
    pub json: Vec<u8>,
    pub layout_vertical: bool,
    pub next_id: usize,
    pub current_page: AppPage,
    pub profile: ProfileConfig,
    pub waypoint_input: String,
    pub selected_saved_route: usize,
    pub inserting_route_at: Option<usize>,
    pub insert_waypoints: String,
    pub confirm_overwrite_route: Option<(usize, usize)>, // (plan_route_idx, workspace_saved_idx)
    pub wind_all_dir: i64,
    pub wind_all_spd: i64,
}

impl Application {
    #[allow(unused_must_use)]
    fn update_data(&mut self) {
        let doc = create_planning(&self.plan);
        let mut pdf_data = vec![];
        self.plan.aircraft_registrations = self.profile.aircraft_registrations.clone();
        self.plan.pics = self.profile.pics.clone();
        self.plan.call_signs = self.profile.call_signs.clone();
        let json_data = serde_json::to_vec_pretty(&self.plan).unwrap_or_default();
        doc.write(&mut pdf_data);
        self.pdf = pdf_data;
        self.json = json_data;
    }

    fn clear_message(&mut self) {
        self.message = None;
    }

    fn toggle_layout(&mut self) {
        self.layout_vertical = !self.layout_vertical;
    }

    fn get_leg(&mut self, (route_idx, leg_idx): (usize, usize)) -> &mut Leg {
        &mut self.plan.routes[route_idx].legs[leg_idx]
    }

    fn get_last_leg_to(&self, route_idx: usize) -> Option<String> {
        self.plan
            .routes
            .get(route_idx)
            .and_then(|route| route.legs.last())
            .map(|leg| leg.to.clone())
    }

    fn get_previous_leg_to(&self, (route_idx, leg_idx): (usize, usize)) -> Option<String> {
        if leg_idx == 0 {
            None
        } else {
            self.plan
                .routes
                .get(route_idx)
                .and_then(|route| route.legs.get(leg_idx - 1))
                .map(|leg| leg.to.clone())
        }
    }

    fn delete_route(&mut self, idx: usize) {
        self.plan.routes.remove(idx);
    }

    fn append_route(&mut self, route: Route) {
        self.plan.routes.push(route);
    }

    fn insert_route(&mut self, idx: usize, route: Route) {
        self.plan.routes.insert(idx, route);
    }

    fn append_leg(&mut self, route_idx: usize, leg: Leg) {
        self.plan.routes[route_idx].legs.push(leg);
    }

    fn delete_leg(&mut self, (route_idx, leg_idx): (usize, usize)) {
        self.plan.routes[route_idx].legs.remove(leg_idx);
    }

    fn insert_leg(&mut self, (route_idx, leg_idx): (usize, usize), leg: Leg) {
        self.plan.routes[route_idx].legs.insert(leg_idx, leg);
    }

    fn set_note_font_bold(&mut self, (route_idx, note_idx): (usize, usize)) {
        let value = self.plan.routes[route_idx].notes[note_idx].string_value();

        if let Some(value) = value {
            self.plan.routes[route_idx].notes[note_idx] = FontType::Bold(value.to_owned());
        }
    }

    fn set_note_font_italics(&mut self, (route_idx, note_idx): (usize, usize)) {
        let value = self.plan.routes[route_idx].notes[note_idx].string_value();

        if let Some(value) = value {
            self.plan.routes[route_idx].notes[note_idx] = FontType::Italics(value.to_owned());
        }
    }

    fn set_note_font_normal(&mut self, (route_idx, note_idx): (usize, usize)) {
        let value = self.plan.routes[route_idx].notes[note_idx].string_value();

        if let Some(value) = value {
            self.plan.routes[route_idx].notes[note_idx] = FontType::Normal(value.to_owned());
        }
    }

    fn update_note(&mut self, (route_idx, note_idx): (usize, usize), note: String) {
        let current = &self.plan.routes[route_idx].notes[note_idx];
        self.plan.routes[route_idx].notes[note_idx] = current.set_value(note);
    }

    fn delete_note(&mut self, (route_idx, note_idx): (usize, usize)) {
        self.plan.routes[route_idx].notes.remove(note_idx);
    }

    fn insert_note(&mut self, (route_idx, note_idx): (usize, usize), note: FontType) {
        self.plan.routes[route_idx].notes.insert(note_idx, note);
    }

    fn append_note(&mut self, route_idx: usize, note: FontType) {
        self.plan.routes[route_idx].notes.push(note);
    }

    fn append_diversion(&mut self, diversion: Diversion) {
        self.plan.diversions.push(diversion);
    }

    fn delete_diversion(&mut self, idx: usize) {
        self.plan.diversions.remove(idx);
    }

    fn insert_diversion(&mut self, idx: usize, diversion: Diversion) {
        self.plan.diversions.insert(idx, diversion);
    }

    fn get_diversion(&mut self, idx: usize) -> &mut Diversion {
        &mut self.plan.diversions[idx]
    }

    fn get_hold(&mut self, idx: usize) -> &mut Hold {
        &mut self.plan.holds[idx]
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

fn handle_create_initial_route(app: &mut Application) {
    // 1. Parse waypoint CSV
    let waypoints: Vec<String> = app
        .waypoint_input
        .split(',')
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect();

    if waypoints.len() < 2 {
        app.message = Some("Need at least 2 waypoints".to_owned());
        return;
    }

    // 2. Create legs from consecutive waypoint pairs using workspace defaults
    let mut legs = Vec::new();
    for i in 0..waypoints.len() - 1 {
        legs.push(Leg {
            from: waypoints[i].clone(),
            to: waypoints[i + 1].clone(),
            safe: app.profile.default_leg_values.safe.clone(),
            planned: app.profile.default_leg_values.planned.clone(),
            speed: app.profile.default_leg_values.speed,
            course: app.profile.default_leg_values.course,
            distance: app.profile.default_leg_values.distance,
            variation: app.profile.default_leg_values.variation,
            wind_direction: app.profile.default_leg_values.wind_direction,
            wind_speed: app.profile.default_leg_values.wind_speed,
        });
    }

    // 3. Create route and update plan
    app.plan.routes.push(Route {
        name: String::new(),
        legs,
        notes: vec![FontType::Blank],
    });

    // 4. Clear input and update
    app.waypoint_input = String::new();
    app.update_data();
}

fn route_save_name(route: &Route, route_idx: usize) -> String {
    if !route.name.is_empty() {
        return route.name.clone();
    }
    match route.legs.as_slice() {
        [] => format!("Route {}", route_idx + 1),
        [leg] => format!("{} to {}", leg.from, leg.to),
        [first, .., last] => format!("{} to {}", first.from, last.to),
    }
}

fn handle_save_route_to_workspace(app: &mut Application, route_idx: usize) {
    if let Some(route) = app.plan.routes.get(route_idx) {
        let name = route_save_name(route, route_idx);
        // Check if a saved route with the same name already exists
        let existing = app.profile.saved_routes.iter().position(|s| s.name == name);
        if let Some(workspace_idx) = existing {
            app.confirm_overwrite_route = Some((route_idx, workspace_idx));
        } else {
            app.profile.saved_routes.push(SavedRoute {
                name,
                waypoints: String::new(),
                legs: route.legs.clone(),
                notes: route.notes.clone(),
            });
            workspace_storage::save_profile_to_local_storage(&app.profile);
        }
    }
}

fn handle_confirm_overwrite(app: &mut Application) {
    if let Some((route_idx, workspace_idx)) = app.confirm_overwrite_route.take() {
        if let Some(route) = app.plan.routes.get(route_idx) {
            let name = route_save_name(route, route_idx);
            if let Some(saved) = app.profile.saved_routes.get_mut(workspace_idx) {
                saved.name = name;
                saved.legs = route.legs.clone();
                saved.notes = route.notes.clone();
                saved.waypoints = String::new();
            }
            workspace_storage::save_profile_to_local_storage(&app.profile);
        }
    }
}

fn handle_create_inserted_route(app: &mut Application) {
    let insert_idx = match app.inserting_route_at {
        Some(idx) => idx,
        None => return,
    };

    // 1. Parse waypoint CSV
    let waypoints: Vec<String> = app
        .insert_waypoints
        .split(',')
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect();

    if waypoints.len() < 2 {
        app.message = Some("Need at least 2 waypoints".to_owned());
        return;
    }

    // 2. Create legs from consecutive waypoint pairs using workspace defaults
    let mut legs = Vec::new();
    for i in 0..waypoints.len() - 1 {
        legs.push(Leg {
            from: waypoints[i].clone(),
            to: waypoints[i + 1].clone(),
            safe: app.profile.default_leg_values.safe.clone(),
            planned: app.profile.default_leg_values.planned.clone(),
            speed: app.profile.default_leg_values.speed,
            course: app.profile.default_leg_values.course,
            distance: app.profile.default_leg_values.distance,
            variation: app.profile.default_leg_values.variation,
            wind_direction: app.profile.default_leg_values.wind_direction,
            wind_speed: app.profile.default_leg_values.wind_speed,
        });
    }

    // 3. Insert route at the specified position
    app.plan.routes.insert(
        insert_idx,
        Route {
            name: String::new(),
            legs,
            notes: vec![FontType::Blank],
        },
    );

    // 4. Clear insertion state and update
    app.inserting_route_at = None;
    app.insert_waypoints = String::new();
    app.update_data();
}

fn handle_profile_change(app: &mut Application, change: ProfileChange) {
    match change {
        ProfileChange::RegistrationAdd(val) => {
            app.profile.aircraft_registrations.push(val);
        }
        ProfileChange::RegistrationUpdate(idx, val) => {
            if let Some(item) = app.profile.aircraft_registrations.get_mut(idx) {
                *item = val;
            }
        }
        ProfileChange::RegistrationDelete(idx) => {
            if idx < app.profile.aircraft_registrations.len() {
                app.profile.aircraft_registrations.remove(idx);
            }
        }
        ProfileChange::PicAdd(val) => app.profile.pics.push(val),
        ProfileChange::PicUpdate(idx, val) => {
            if let Some(item) = app.profile.pics.get_mut(idx) {
                *item = val;
            }
        }
        ProfileChange::PicDelete(idx) => {
            if idx < app.profile.pics.len() {
                app.profile.pics.remove(idx);
            }
        }
        ProfileChange::CallSignAdd(val) => app.profile.call_signs.push(val),
        ProfileChange::CallSignUpdate(idx, val) => {
            if let Some(item) = app.profile.call_signs.get_mut(idx) {
                *item = val;
            }
        }
        ProfileChange::CallSignDelete(idx) => {
            if idx < app.profile.call_signs.len() {
                app.profile.call_signs.remove(idx);
            }
        }
        ProfileChange::DefaultSpeed(val) => {
            app.profile.default_leg_values.speed = val;
        }
        ProfileChange::DefaultCourse(val) => {
            app.profile.default_leg_values.course = val;
        }
        ProfileChange::DefaultDistance(val) => {
            app.profile.default_leg_values.distance = val;
        }
        ProfileChange::DefaultVariation(val) => {
            app.profile.default_leg_values.variation = val;
        }
        ProfileChange::DefaultWindDirection(val) => {
            app.profile.default_leg_values.wind_direction = val;
        }
        ProfileChange::DefaultWindSpeed(val) => {
            app.profile.default_leg_values.wind_speed = val;
        }
        ProfileChange::DefaultSafe(val) => {
            app.profile.default_leg_values.safe = val;
        }
        ProfileChange::DefaultPlanned(val) => {
            app.profile.default_leg_values.planned = val;
        }
        ProfileChange::SavedRouteAdd => {
            app.profile.saved_routes.push(SavedRoute::default());
        }
        ProfileChange::SavedRouteDelete(idx) => {
            if idx < app.profile.saved_routes.len() {
                app.profile.saved_routes.remove(idx);
            }
        }
        ProfileChange::SavedRouteLoadToPlan(idx) => {
            if let Some(saved) = app.profile.saved_routes.get(idx) {
                app.plan.routes.push(Route {
                    name: saved.name.clone(),
                    legs: saved.legs.clone(),
                    notes: saved.notes.clone(),
                });
                app.current_page = AppPage::FlightPlanning;
                app.update_data();
            }
        }
        ProfileChange::SavedRouteName(idx, val) => {
            if let Some(route) = app.profile.saved_routes.get_mut(idx) {
                route.name = val;
            }
        }
        ProfileChange::SavedRouteWaypoints(idx, val) => {
            if let Some(route) = app.profile.saved_routes.get_mut(idx) {
                route.waypoints = val;
            }
        }
        ProfileChange::SavedHoldAdd => {
            app.profile.saved_holds.push(SavedHold::default());
        }
        ProfileChange::SavedHoldDelete(idx) => {
            if idx < app.profile.saved_holds.len() {
                app.profile.saved_holds.remove(idx);
            }
        }
        ProfileChange::SavedHoldLoadToPlan(idx) => {
            if let Some(saved) = app.profile.saved_holds.get(idx) {
                app.plan.holds.push(Hold {
                    description: saved.description.clone(),
                    right_hand: saved.right_hand,
                    in_bound_track: saved.in_bound_track,
                    aircraft_speed: saved.aircraft_speed,
                    variation: saved.variation,
                    wind: Velocity {
                        angle: saved.wind_angle,
                        speed: saved.wind_speed,
                    },
                });
                app.current_page = AppPage::FlightPlanning;
                app.update_data();
            }
        }
        ProfileChange::SavedHoldName(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.name = val;
            }
        }
        ProfileChange::SavedHoldDescription(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.description = val;
            }
        }
        ProfileChange::SavedHoldRightHand(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.right_hand = val;
            }
        }
        ProfileChange::SavedHoldInBoundTrack(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.in_bound_track = val;
            }
        }
        ProfileChange::SavedHoldSpeed(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.aircraft_speed = val;
            }
        }
        ProfileChange::SavedHoldVariation(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.variation = val;
            }
        }
        ProfileChange::SavedHoldWindDirection(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.wind_angle = val;
            }
        }
        ProfileChange::SavedHoldWindSpeed(idx, val) => {
            if let Some(hold) = app.profile.saved_holds.get_mut(idx) {
                hold.wind_speed = val;
            }
        }
    }

    workspace_storage::save_profile_to_local_storage(&app.profile);
}

fn submit_profile_load(app: &mut Application, file: File, link: Scope<Application>) {
    let file_name = file.name();
    let id = app.get_next_id();
    let task = {
        read_as_bytes(&file, move |data| {
            let details = LoadedFileDetails {
                id,
                file_name,
                data,
            };
            link.send_message(PlanMessage::ProfileLoaded(details))
        })
    };
    app.readers.insert(id, task);
}

fn update_profile(app: &mut Application, details: LoadedFileDetails) {
    match decode_profile(app, details) {
        Ok(workspace) => {
            app.profile = workspace;
            workspace_storage::save_profile_to_local_storage(&app.profile);
        }
        Err(err) => app.message = Some(err.to_err_string()),
    }
}

fn decode_profile(
    app: &mut Application,
    details: LoadedFileDetails,
) -> Result<ProfileConfig, KneeboardError> {
    let LoadedFileDetails {
        id,
        file_name,
        data,
    } = details;
    app.readers.remove(&id);

    let data = if let Ok(data) = data {
        data
    } else {
        return Err(KneeboardError::String(
            "Failed to read loaded file".to_owned(),
        ));
    };

    let file_name = file_name.to_lowercase();
    let is_json = file_name.ends_with(".json") || file_name.ends_with(".jsn");

    if is_json {
        Ok(serde_json::from_reader(&data[..])?)
    } else {
        Err(KneeboardError::String(
            "Profile files must be JSON format".to_owned(),
        ))
    }
}
