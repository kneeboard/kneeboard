use crate::application::Application;
use crate::common::{to_string, IsLast};

use crate::messages::{PlanChange, PlanMessage};

use definition::FontType;

use web_sys::Event;

use yew::prelude::*;

pub fn notes_html(ctx: &Context<Application>, route_idx: usize, notes: &[FontType]) -> Html {
    let notes_html: Html = notes
        .iter()
        .enumerate()
        .map(|(note_idx, note)| {
            note_html(ctx, note, (route_idx, note_idx), notes.is_last(note_idx))
        })
        .collect();

    let append_callback = ctx.link().callback(move |_| on_click_append(route_idx));

    html!(
        <>
            {notes_html}
            <button class="add-row" onclick={append_callback}>{"+ Note"}</button>
        </>
    )
}

fn note_html(
    ctx: &Context<Application>,
    note: &FontType,
    idx: (usize, usize),
    _is_last: bool,
) -> Html {
    let link = ctx.link();
    let value = note.string_value().unwrap_or("").to_owned();
    let value_callback = link.callback(move |e| on_change_value(e, idx));
    let delete_callback = link.callback(move |_| on_click_delete(idx));

    let (is_bold, is_italic) = match note {
        FontType::Bold(_) => (true, false),
        FontType::Italics(_) => (false, true),
        _ => (false, false),
    };

    let bold_class = if is_bold { "toggle on" } else { "toggle" };
    let italic_class = if is_italic { "toggle on" } else { "toggle" };

    let bold_callback = link.callback(move |_| {
        if is_bold {
            on_click_normal(idx)
        } else {
            on_click_bold(idx)
        }
    });

    let italic_callback = link.callback(move |_| {
        if is_italic {
            on_click_normal(idx)
        } else {
            on_click_italics(idx)
        }
    });

    html!(
        <div class="note-row">
            <button class={bold_class} onclick={bold_callback}>{"B"}</button>
            <button class={italic_class} onclick={italic_callback}>{"I"}</button>
            <input class="note-txt" type="text" value={value} onchange={value_callback}/>
            <button class="ibtn del" onclick={delete_callback}>{"×"}</button>
        </div>
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

fn on_click_append(route_idx: usize) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteAppend(route_idx, FontType::Blank))
}

fn on_change_value(e: Event, idx: (usize, usize)) -> PlanMessage {
    let value = to_string(e);
    PlanMessage::DataChange(PlanChange::NoteUpdate(idx, value))
}

#[allow(dead_code)]
fn on_click_insert(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteInsert(idx, FontType::Blank))
}

fn on_click_delete(idx: (usize, usize)) -> PlanMessage {
    PlanMessage::DataChange(PlanChange::NoteDelete(idx))
}
