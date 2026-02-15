use definition::FontType;

use gloo::file::{File, FileReadError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppPage {
    #[default]
    FlightPlanning,
    Workspace,
    About,
}

pub enum PlanMessage {
    Files(Option<File>),
    Loaded(LoadedFileDetails),
    DataChange(PlanChange),
    SetMessage(String),
    LayoutToggle,
    ThemeToggle,

    // Navigation
    NavigateTo(AppPage),

    // Initial Route Creation
    InitialWaypointsInput(String),
    CreateInitialRoute,
    SelectSavedRoute(usize),

    // Save/Load routes to/from workspace
    SaveRouteToWorkspace(usize),
    ConfirmOverwriteSavedRoute,
    CancelOverwriteSavedRoute,

    // Route Insertion with Waypoints
    ShowRouteInsertDialog(usize),
    ShowRouteInsertBelowDialog(usize),
    InsertRouteWaypoints(String),
    CreateInsertedRoute,
    CancelRouteInsert,

    // Workspace Management
    WorkspaceLoadFile(Option<File>),
    WorkspaceLoaded(LoadedFileDetails),
    WorkspaceChange(WorkspaceChange),
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
    Field1(Option<String>),
    Field2(Option<String>),
    Field3(Option<String>),
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
    RouteName(usize, String),
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

    HoldAppend,
    HoldDelete(usize),
    HoldDescription(usize, String),
    HoldRightHand(usize, bool),
    HoldInBoundTrack(usize, i64),
    HoldSpeed(usize, i64),
    HoldVariation(usize, i64),
    HoldWindDirection(usize, i64),
    HoldWindSpeed(usize, i64),

    SetWindAllDir(i64),
    SetWindAllSpd(i64),
    ApplyWindAll,

    RouteAppend,

    // Fill entire column for a route
    RouteFillSafe(usize, String),
    RouteFillPlanned(usize, String),
    RouteFillSpeed(usize, i64),
    RouteFillCourse(usize, i64),
    RouteFillDistance(usize, i64),
    RouteFillVariation(usize, i64),
    RouteFillWindDirection(usize, i64),
    RouteFillWindSpeed(usize, i64),
}

#[derive(Debug)]
pub enum WorkspaceChange {
    // Aircraft registrations
    RegistrationAdd(String),
    RegistrationUpdate(usize, String),
    RegistrationDelete(usize),

    // PICs
    PicAdd(String),
    PicUpdate(usize, String),
    PicDelete(usize),

    // Call signs
    CallSignAdd(String),
    CallSignUpdate(usize, String),
    CallSignDelete(usize),

    // Default leg values
    DefaultSpeed(i64),
    DefaultCourse(i64),
    DefaultDistance(i64),
    DefaultVariation(i64),
    DefaultWindDirection(i64),
    DefaultWindSpeed(i64),
    DefaultSafe(String),
    DefaultPlanned(String),

    // Saved routes
    SavedRouteAdd,
    SavedRouteDelete(usize),
    SavedRouteLoadToPlan(usize),
    SavedRouteName(usize, String),
    SavedRouteWaypoints(usize, String),

    // Saved holds
    SavedHoldAdd,
    SavedHoldDelete(usize),
    SavedHoldLoadToPlan(usize),
    SavedHoldName(usize, String),
    SavedHoldDescription(usize, String),
    SavedHoldRightHand(usize, bool),
    SavedHoldInBoundTrack(usize, i64),
    SavedHoldSpeed(usize, i64),
    SavedHoldVariation(usize, i64),
    SavedHoldWindDirection(usize, i64),
    SavedHoldWindSpeed(usize, i64),
}
