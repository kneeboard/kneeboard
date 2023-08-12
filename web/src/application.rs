use crate::common::to_files;
use crate::detail::details_html;
use crate::diversion::diversion_html;
use crate::important::important_html;
use crate::messages::{LoadedFileDetails, PlanChange, PlanMessage};
use crate::route::routes_html;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use common::{
    create_template_diversion, create_template_leg, create_template_plan, create_template_route,
    KneeboardError,
};
use core::planner::create_planning;
use definition::{Diversion, FontType, Leg, Plan, Route};
use gloo_console::__macro::JsValue;

use gloo::file::callbacks::read_as_bytes;
use gloo::file::{callbacks::FileReader, File};

use crate::icons::{file_earmark_arrow_down, file_earmark_arrow_up, file_pdf, layout_text_sidebar};
use std::collections::HashMap;
use web_sys::{Event, FileList};
use yew::{html::Scope, prelude::*};

impl Component for Application {
    type Message = PlanMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let plan = create_template_plan();
        let mut app = Application {
            plan,
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
            PlanMessage::SetMessage(msg) => self.message = Some(msg),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let form_html = main_form(self, ctx);
        let pdf_html = pdf_html(self);

        let under_development = html!(
            <div class="alert alert-warning" role="alert">
              <table width="100%"><tr><td align="middle"><h4><i>{"Under Development"}</i></h4></td></tr></table>
            </div>
        );

        if self.layout_vertical {
            html!(
                <div id="wrapper">
                  {under_development}
                  if let Some(msg) = &self.message {

                    <div class="alert alert-danger" role="alert">
                    {msg}
                    </div>
                  }
                  <table style="width:100%">
                    <tr>
                       <td valign="top">{form_html}</td>
                    </tr>
                    <tr>
                      <td>{pdf_html}</td>
                    </tr>
                </table>
            </div>
            )
        } else {
            html!(
                <div id="wrapper">
                  {under_development}
                  if let Some(msg) = &self.message {
                    <div class="alert alert-danger" role="alert">
                    {msg}
                    </div>
                  }
                  <table style="width:100%">
                    <tr>
                      <td valign="top">{form_html}</td>
                      <td style="width:33%; vertical-align:top">{pdf_html}</td>
                    </tr>
                  </table>
                </div>
            )
        }
    }
}

fn pdf_html(app: &Application) -> Html {
    html!(
        <embed title="kneeboard-notes.pdf" type="application/pdf" width="100%" height="800" src={pdf_url(app, "application/pdf")}/>
    )
}

fn pdf_url(app: &Application, mime: &str) -> String {
    let pdf_base64 = STANDARD_NO_PAD.encode(&app.pdf);
    format!("data:{mime};base64,{pdf_base64}")
}

fn main_form(app: &Application, ctx: &Context<Application>) -> Html {
    fn on_click_toggle_layout(_: MouseEvent) -> PlanMessage {
        PlanMessage::LayoutToggle
    }

    fn on_click_upload(e: Event) -> PlanMessage {
        upload_files(to_files(e))
    }

    let yml_base64 = STANDARD_NO_PAD.encode(&app.yml);

    let details_html = details_html(ctx, &app.plan.detail);
    let important_html = important_html(ctx, &app.plan.important);
    let routes_html = routes_html(ctx, &app.plan.routes);
    let deviation_html = diversion_html(ctx, &app.plan.diversions);

    let encoded_yml = format!("data:text/plain;base64,{yml_base64}");

    html!(
        <>
        <table width="100%">
          <tr>
            <td width="32px">
              <div class="image-upload">
                <label for="fileToUpload" title="Load notes">
                  {file_earmark_arrow_up(48)}
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
            </td>
            <td width="32px">
              <a download="kneeboard-notes.yml" title="Save notes" href={encoded_yml}>
               {file_earmark_arrow_down(48)}
              </a>
            </td>
            <td width="32px">
              <a download="kneeboard-notes.pdf" title="Download PDF"  href={pdf_url(app, "application/octet-stream")}>
               {file_pdf(48)}
              </a>
            </td>
            <td align="right">
              <button class="btn btn-link" onclick={ctx.link().callback(on_click_toggle_layout)}>
                {layout_text_sidebar(48)}
              </button>
            </td>
          </tr>
        </table>
        <br/>
        <table width="100%">
          <tr>
            <td width="25%">
              {important_html}
            </td>
            <td width="50%"></td>
            <td width="25%" align="right">
              {details_html}
            </td>
          </tr>
        </table>

        <h4>{"Routes:"}</h4>
        {routes_html}
        <h4>{"Diversions:"}</h4>
        {deviation_html}
        </>
    )
}

fn handle_plan_change(app: &mut Application, change: PlanChange) {
    fn clamp_speed(value: i64) -> i64 {
        value.max(20)
    }

    match change {
        PlanChange::Important(idx, value) => app.plan.important.lines[idx] = value,
        PlanChange::Tail(tail) => app.plan.detail.tail = tail,
        PlanChange::PilotInCommand(pic) => app.plan.detail.pic = pic,
        PlanChange::CallSign(call_sign) => app.plan.detail.call_sign = call_sign,

        PlanChange::LegFrom(idx, value) => app.get_leg(idx).from = value,
        PlanChange::LegTo(idx, value) => app.get_leg(idx).to = value,
        PlanChange::LegSafe(idx, value) => app.get_leg(idx).safe = value,
        PlanChange::LegPlanned(idx, value) => app.get_leg(idx).planned = value,
        // Clamp the airspeed to above 20
        PlanChange::LegSpeed(idx, value) => app.get_leg(idx).speed = clamp_speed(value),
        PlanChange::LegCourse(idx, value) => app.get_leg(idx).course = value,
        PlanChange::LegDistance(idx, value) => app.get_leg(idx).distance = value,
        PlanChange::LegVariation(idx, value) => app.get_leg(idx).variation = value,
        PlanChange::LegWindDirection(idx, value) => app.get_leg(idx).wind_direction = value,
        PlanChange::LegWindSpeed(idx, value) => app.get_leg(idx).wind_speed = value,
        PlanChange::LegAppend(route_idx) => app.append_leg(route_idx, create_template_leg()),
        PlanChange::LegDelete(idx) => app.delete_leg(idx),
        PlanChange::LegInsert(idx) => app.insert_leg(idx, create_template_leg()),

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
        // Clamp the airspeed to above 20
        PlanChange::DiversionSpeed(idx, value) => {
            app.get_diversion(idx).aircraft_speed = clamp_speed(value)
        }
        PlanChange::DiversionVariation(idx, value) => app.get_diversion(idx).variation = value,
        PlanChange::DiversionWindDirection(idx, value) => app.get_diversion(idx).wind.angle = value,
        PlanChange::DiversionWindSpeed(idx, value) => app.get_diversion(idx).wind.speed = value,
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
        Ok(plan) => {
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

    let (is_json, is_yaml) = {
        let file_name = file_name.to_lowercase();
        let is_json = file_name.ends_with(".json") || file_name.ends_with(".jsn");
        let is_yaml = file_name.ends_with(".yaml") || file_name.ends_with(".yml");

        (is_json, is_yaml)
    };

    if is_json {
        Ok(serde_json::from_reader(&data[..])?)
    } else if is_yaml {
        Ok(serde_yaml::from_reader(&data[..])?)
    } else {
        return Err(KneeboardError::String("Unsupported file type".to_owned()));
    }
}

fn submit_load(app: &mut Application, file: File, link: Scope<Application>) {
    let file_name = file.name();

    let id = app.get_next_id();
    let task = {
        let file_name = file_name;

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
    plan: Plan,
    message: Option<String>,
    readers: HashMap<usize, FileReader>,
    pdf: Vec<u8>,
    yml: Vec<u8>,
    layout_vertical: bool,
    next_id: usize,
}

impl Application {
    #[allow(unused_must_use)]
    fn update_data(&mut self) {
        let doc = create_planning(&self.plan);
        let mut pdf_data = vec![];
        let mut yml_data = vec![];
        serde_yaml::to_writer(&mut yml_data, &self.plan);
        doc.write(&mut pdf_data);
        self.pdf = pdf_data;
        self.yml = yml_data;
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

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
