use core::definition::FontType;

use gloo::file::{File, FileReadError};

pub enum PlanMessage {
    Files(Option<File>),
    Loaded(LoadedFileDetails),
    DataChange(PlanChange),
    SetMessage(String),
    LayoutToggle,
}

pub struct LoadedFileDetails {
    pub id: usize,
    pub file_name: String,
    pub data: Result<Vec<u8>, FileReadError>,
}

#[derive(Debug)]
pub enum PlanChange {
    Tail(Option<String>),
    PilotInCommand(Option<String>),
    CallSign(Option<String>),
    LegFrom((usize, usize), String),
    LegTo((usize, usize), String),
    LegSafe((usize, usize), String),
    LegPlanned((usize, usize), String),
    LegSpeed((usize, usize), i64),
    LegCourse((usize, usize), i64),
    LegDistance((usize, usize), i64),
    LegVariation((usize, usize), i64),
    LegWindDirection((usize, usize), i64),
    LegWindSpeed((usize, usize), i64),
    LegAppend(usize),
    LegDelete((usize, usize)),
    LegInsert((usize, usize)),
    RouteDelete(usize),
    RouteInsert(usize),
    NoteBold((usize, usize)),
    NoteItalics((usize, usize)),
    NoteNormal((usize, usize)),
    NoteUpdate((usize, usize), String),
    NoteInsert((usize, usize), FontType),
    NoteAppend(usize, FontType),
    NoteDelete((usize, usize)),

    DiversionInsert(usize),
    DiversionAppend,
    DiversionDelete(usize),
    DiversionSpeed(usize, i64),
    DiversionVariation(usize, i64),
    DiversionWindDirection(usize, i64),
    DiversionWindSpeed(usize, i64),

    RouteAppend,
}
