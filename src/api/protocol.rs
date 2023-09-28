use yaserde_derive::{YaDeserialize, YaSerialize};


#[derive(Debug)]
pub enum ProtocolPacket {
    Joined(Joined),
    Join(Join),
    Room(Room),
    Left(Left),
    JoinPrepared(JoinPrepared),
    ErrorPacket(ErrorPacket),
    Close(Close),
}


#[derive(Clone, Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(rename = "protocol")]
pub struct Protocol {
    #[yaserde(rename = "joined")]
    pub joined: Option<Joined>,

    #[yaserde(rename = "join")]
    pub join: Option<Join>,

    #[yaserde(rename = "room")]
    pub room: Option<Room>,

    #[yaserde(rename = "left")]
    pub left: Option<Left>,

    #[yaserde(rename = "joinPrepared")]
    join_prepared: Option<JoinPrepared>,

    #[yaserde(rename = "errorpacket")]
    error_packet: Option<ErrorPacket>,

    #[yaserde(rename = "close")]
    close: Option<Close>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "join")]
pub struct Join {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "room")]
pub struct Room {
    #[yaserde(attribute, rename = "roomId")]
    pub room_id: String,

    #[yaserde(rename = "data")]
    pub data: Data,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "data")]
pub struct Data {
    #[yaserde(attribute)]
    pub class: DataClass,

    #[yaserde(attribute)]
    pub color: Option<String>,

    #[yaserde(rename = "state")]
    pub state: Option<State>,

    #[yaserde(flatten)]
    pub _move: Option<Move>,

    pub definition: Option<Definition>,

    pub scores: Option<Scores>,

    pub winner: Option<Winner>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub enum DataClass {
    #[default]
    #[yaserde(rename = "welcomeMessage")]
    WelcomeMessage,
    #[yaserde(rename = "momento")]
    Memento,
    #[yaserde(rename = "moveRequest")]
    MoveRequest,
    #[yaserde(rename = "move")]
    Move,
    #[yaserde(rename = "result")]
    Result,
    #[yaserde(rename = "error")]
    Error,
}


#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Move {
    #[yaserde(rename = "from", child)]
    pub from: Coordinate,
    #[yaserde(rename = "to", child)]
    pub to: Coordinate,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Coordinate {
    #[yaserde(attribute)]
    pub q: i32,

    #[yaserde(attribute)]
    pub r: i32,

    #[yaserde(attribute)]
    pub s: i32,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub enum CubeDirection {
    #[default]
    #[yaserde(rename = "RIGHT")]
    Right,
    #[yaserde(rename = "UP_RIGHT")]
    UpRight,
    #[yaserde(rename = "UP")]
    Up,
    #[yaserde(rename = "UP_LEFT")]
    UpLeft,
    #[yaserde(rename = "LEFT")]
    Left,
    #[yaserde(rename = "DOWN_LEFT")]
    DownLeft,
    #[yaserde(rename = "DOWN")]
    Down,
    #[yaserde(rename = "DOWN_RIGHT")]
    DownRight,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Water;

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Island;

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Sandbank;

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Passenger {
    #[yaserde(attribute)]
    pub direction: CubeDirection,
    #[yaserde(attribute)]
    pub passenger: i32,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Goal;


#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "field-array")]
pub struct FieldArray {
    #[yaserde(rename = "water")]
    pub water: Vec<Water>,
    #[yaserde(rename = "island")]
    pub island: Vec<Island>,
    #[yaserde(rename = "sandbank")]
    pub sandbank: Vec<Sandbank>,
    #[yaserde(rename = "passenger")]
    pub passenger: Vec<Passenger>,
    #[yaserde(rename = "goal")]
    pub goal: Vec<Goal>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Ship {
    #[yaserde(attribute)]
    pub team: String,

    #[yaserde(attribute)]
    pub direction: CubeDirection,

    #[yaserde(attribute)]
    pub speed: i32,

    #[yaserde(attribute)]
    pub coal: i32,

    #[yaserde(attribute)]
    pub passengers: i32,

    #[yaserde(attribute, rename = "freeTurns")]
    pub free_turns: i32,

    #[yaserde(attribute)]
    pub points: i32,

    pub  position: Coordinate,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "state")]
pub struct State {
    #[yaserde(attribute)]
    pub class: String,

    #[yaserde(attribute, rename = "startTeam")]
    pub start_team: String,

    #[yaserde(attribute)]
    pub turn: i32,

    #[yaserde(attribute, rename = "currentTeam")]
    pub current_team: String,

    pub board: Board,

    pub ship: Vec<Ship>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Segment {
    #[yaserde(attribute)]
    pub direction: CubeDirection,

    pub center: Coordinate,

    #[yaserde(rename = "field-array")]
    pub field_array: Vec<FieldArray>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "board")]
pub struct Board {
    #[yaserde(attribute, rename = "nextDirection")]
    pub next_direction: CubeDirection,

    pub segment: Vec<Segment>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "list")]
pub struct List {
    #[yaserde(rename = "field")]
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "fishes")]
pub struct Fishes {
    #[yaserde(rename = "int")]
    pub ints: Vec<String>,
}


#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "definition")]
pub struct Definition {
    #[yaserde(rename = "fragment")]
    fragments: Vec<Fragment>,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "fragment")]
pub struct Fragment {
    #[yaserde(attribute)]
    name: String,
    aggregation: String,
    relevant_for_ranking: bool,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "scores")]
pub struct Scores {
    #[yaserde(rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "entry")]
pub struct Entry {
    player: Player,
    score: Score,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "player")]
pub struct Player {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    team: String,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "score")]
pub struct Score {
    #[yaserde(attribute)]
    cause: String,
    #[yaserde(attribute)]
    reason: String,
    #[yaserde(rename = "part")]
    parts: Vec<i32>,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "winner")]
pub struct Winner {
    #[yaserde(attribute)]
    team: String,
}

#[derive(Clone, Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(rename = "left")]
pub struct Left {
    #[yaserde(attribute)]
    pub room_id: String,
}

#[derive(Clone, Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(rename = "joined")]
pub struct Joined {
    #[yaserde(attribute, rename = "roomId")]
    pub room_id: String,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "joinPrepared")]
pub struct JoinPrepared {
    #[yaserde(attribute, rename = "reservationCode")]
    pub reservation_code: String,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "errorpacket")]
pub struct ErrorPacket {
    #[yaserde(attribute, rename = "message")]
    pub message: String,

    #[yaserde(rename = "originalRequest")]
    original_request: OriginalRequest,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "originalRequest")]
pub struct OriginalRequest {
    #[yaserde(attribute, rename = "class")]
    class: String,

    #[yaserde(attribute, rename = "reservationCode")]
    reservation_code: String,
}

#[derive(Clone, Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
#[yaserde(rename = "close")]
pub struct Close {}