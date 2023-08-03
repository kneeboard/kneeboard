use crate::application::Application;
use crate::common::{append_insert_delete, to_string, IsLast};

use crate::messages::{PlanChange, PlanMessage};

use definition::FontType;

use crate::icons::{chevron_bar_down, type_bold, type_italic};

use web_sys::Event;

use yew::prelude::*;

pub fn notes_html(ctx: &Context<Application>, route_idx: usize, notes: &[FontType]) -> Html {
    let notes_html = if notes.is_empty() {
        let append_callback = ctx.link().callback(move |_| on_click_append(route_idx));
        html!(
            <tr>
              <td align="middle">
                <button type="button" class="btn btn-link btn-sm py-0 px-0" onclick={append_callback}>
                {chevron_bar_down(32)}
                </button>
              </td>
            </tr>
        )
    } else {
        let notes_html: Html = notes
            .iter()
            .enumerate()
            .map(|(note_idx, note)| {
                note_html(ctx, note, (route_idx, note_idx), notes.is_last(note_idx))
            })
            .collect();

        notes_html
    };

    html!(
          <table class="table table-bordered">
            <tbody>
              {notes_html}
            </tbody>
          </table>
    )
}

fn note_html(
    ctx: &Context<Application>,
    note: &FontType,
    idx: (usize, usize),
    is_last: bool,
) -> Html {
    let (route_idx, _) = idx;

    let link = ctx.link();
    let value = note.string_value().unwrap_or("").to_owned();
    let value_callback = link.callback(move |e| on_change_value(e, idx));
    let insert_callback = link.callback(move |_| on_click_insert(idx));
    let delete_callback = link.callback(move |_| on_click_delete(idx));
    let append_callback = link.callback(move |_| on_click_append(route_idx));

    html!(
        <tr>
          <td width="20px" style="padding: 2px" valign="middle">{font_type(ctx, idx, note)}</td>
          <td valign="top" style="padding: 2px">
            <input type="text" class="form-control" value={value} onchange={value_callback}/>
          </td>
          <td valign="top" style="padding: 0px" width="5px">
            {append_insert_delete(append_callback, insert_callback, delete_callback, is_last) }
          </td>
        </tr>
    )
}

fn on_click_bold(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteBold(idx))
}

fn on_click_italics(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteItalics(idx))
}

fn on_click_normal(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteNormal(idx))
}

fn font_type(ctx: &Context<Application>, idx: (usize, usize), note: &FontType) -> Html {
    let (bold_button, italics_button) = match note {
        FontType::Bold(_) => bold(ctx, idx),
        FontType::Italics(_) => italics(ctx, idx),
        _ => normal(ctx, idx),
    };

    html! {
        <table>
            <tr>
                <td width="10px" style="padding: 0px" valign="middle">
                    {italics_button}
                </td>
                <td width="10px" style="padding: 0px" valign="middle">
                    {bold_button}
                </td>
            </tr>
        </table>
    }
}

fn bold(ctx: &Context<Application>, idx: (usize, usize)) -> (Html, Html) {
    let i_class = "btn btn-link btn-sm";
    let b_class = "btn btn-primary btn-sm";

    let bold_button = html! {
        <button type="button" class={b_class} onclick={ctx.link().callback(move |_| on_click_normal(idx))}>
        {type_bold(18)}
        </button>
    };

    let italics_button = html! {
        <button type="button" class={i_class} onclick={ctx.link().callback(move |_| on_click_italics(idx))}>
        {type_italic(18)}
        </button>
    };

    (bold_button, italics_button)
}

fn italics(ctx: &Context<Application>, idx: (usize, usize)) -> (Html, Html) {
    let i_class = "btn btn-primary btn-sm";
    let b_class = "btn btn-link btn-sm";

    let bold_button = html! {
        <button type="button" class={b_class} onclick={ctx.link().callback(move |_| on_click_bold(idx))}>
        {type_bold(18)}
        </button>
    };

    let italics_button = html! {
        <button type="button" class={i_class} onclick={ctx.link().callback(move |_| on_click_normal(idx))}>
        {type_italic(18)}
        </button>
    };

    (bold_button, italics_button)
}

fn normal(ctx: &Context<Application>, idx: (usize, usize)) -> (Html, Html) {
    let i_class = "btn btn-link btn-sm";
    let b_class = "btn btn-link btn-sm";

    let bold_button = html! {
        <button type="button" class={b_class} onclick={ctx.link().callback(move |_| on_click_bold(idx))}>
        {type_bold(18)}
        </button>
    };

    let italics_button = html! {
        <button type="button" class={i_class} onclick={ctx.link().callback(move |_| on_click_italics(idx))}>
        {type_italic(18)}
        </button>
    };

    (bold_button, italics_button)
}

fn on_click_append(route_idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteAppend(route_idx, FontType::Blank))
}

fn on_change_value(e: Event, idx: (usize, usize)) -> PlanMessage {
    let value = to_string(e);
    PlanMessage::DataChange(PlanChange::NoteUpdate(idx, value))
}

fn on_click_insert(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteInsert(idx, FontType::Blank))
}

fn on_click_delete(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteDelete(idx))
}
