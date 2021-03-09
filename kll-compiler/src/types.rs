use layouts::Layout;
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

pub type Indices = Vec<Range<usize>>;
pub type Map<'a> = HashMap<&'a str, &'a str>;

pub fn format_indices(ranges: &Indices) -> String {
    ranges
        .iter()
        .map(|range| format!("{}-{}", range.start, range.end))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn maybe_quote(text: &str) -> String {
    if text.contains(' ') {
        format!("\"{}\"", text)
    } else {
        text.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Define((&'a str, &'a str)),
    Variable((&'a str, Option<usize>, &'a str)),
    Capability((&'a str, Capability<'a>)),
    Keymap((Vec<Vec<Trigger<'a>>>, TriggerMode, Vec<Vec<Action<'a>>>)),
    Position((Indices, Position)),
    Pixelmap((Indices, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, Indices, Vec<Pixel<'a>>)),
    NOP,
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define((name, val)) => write!(f, "{} = {};", name, maybe_quote(val)),
            Self::Variable((name, index, val)) => {
                if let Some(index) = index {
                    write!(
                        f,
                        "{}[{}] = {};",
                        maybe_quote(name),
                        index,
                        maybe_quote(val)
                    )
                } else {
                    write!(f, "{} = {};", maybe_quote(name), maybe_quote(val))
                }
            }
            Self::Capability((name, cap)) => write!(f, "{} = {};", name, cap),
            Self::Keymap((triggers, varient, actions)) => {
                write!(f, "{:?} {} {:?};", triggers, varient, actions)
            }
            Self::Position((indices, pos)) => {
                write!(f, "P[{}] <= {};", format_indices(indices), pos)
            }
            Self::Pixelmap((indices, map)) => write!(
                f,
                "P[{}]{} : {};",
                format_indices(indices),
                map.channels
                    .iter()
                    .map(|(c, w)| format!("{}:{}", c, w))
                    .collect::<Vec<String>>()
                    .join(", "),
                map.scancode
                    .map(|x| format!("S{}", x))
                    .unwrap_or_else(|| "None".to_string())
            ),
            Self::Animation((name, anim)) => write!(f, "A[{}] <= {:?};", name, anim.modifiers),
            Self::Frame((name, indices, frame)) => write!(
                f,
                "A[{}, {}] <= {:?};",
                name,
                format_indices(indices),
                frame
            ),
            Self::NOP => Ok(()),
        }
    }
}

#[derive(Debug, Default, Clone, Merge)]
pub struct Position {
    pub x: f32,  // mm
    pub y: f32,  // mm
    pub z: f32,  // mm
    pub rx: f32, // deg
    pub ry: f32, // deg
    pub rz: f32, // deg
}

impl Position {
    pub fn from_map(map: Map) -> Self {
        let mut pos = Position::default();
        for (k, v) in map.iter() {
            let v = v.parse::<f32>().unwrap();
            match *k {
                "x" => pos.x = v,
                "y" => pos.y = v,
                "z" => pos.z = v,
                "rx" => pos.rx = v,
                "ry" => pos.ry = v,
                "rz" => pos.rz = v,
                _ => {}
            }
        }

        pos
    }
}

impl<'a> fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x != 0. {
            write!(f, "x:{}", self.x)?;
        }
        if self.y != 0. {
            write!(f, "y:{}", self.y)?;
        }
        if self.z != 0. {
            write!(f, "z:{}", self.z)?;
        }
        if self.rx != 0. {
            write!(f, "x:{}", self.rx)?;
        }
        if self.ry != 0. {
            write!(f, "y:{}", self.ry)?;
        }
        if self.rz != 0. {
            write!(f, "z:{}", self.rz)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Merge)]
pub struct PixelDef {
    #[combine]
    pub channels: Vec<(usize, usize)>,
    pub scancode: Option<usize>,
}

impl PixelDef {
    pub fn new(channelmap: Map, scancode: Option<usize>) -> Self {
        let channels = channelmap
            .iter()
            .map(|(k, v)| {
                let k = k.parse::<usize>().unwrap();
                let v = v.parse::<usize>().unwrap();
                (k, v)
            })
            .collect::<Vec<_>>();

        PixelDef { scancode, channels }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Animation<'a> {
    pub modifiers: Map<'a>,
    pub frames: Vec<Vec<Pixel<'a>>>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Capability<'a> {
    pub function: &'a str,
    //pub args: Map<'a>, // XXX: Can't hash a HashMap
    pub args: Vec<&'a str>,
}

impl<'a> fmt::Display for Capability<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", self.function, self.args)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LayerMode {
    Normal,
    Shift,
    Latch,
    Lock,
}

impl LayerMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Layer" => Self::Normal,
            "LayerShift" => Self::Shift,
            "LayerLatch" => Self::Latch,
            "LayerLock" => Self::Lock,
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for LayerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Layer"),
            Self::Shift => write!(f, "LayerShift"),
            Self::Latch => write!(f, "LayerLatch"),
            Self::Lock => write!(f, "LayerLock"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerType<'a> {
    Key(Key<'a>),
    Layer((LayerMode, Indices)),
    Indicator(Indices),
    Generic((usize, usize, Option<usize>)),
    Animation(&'a str),
}

impl<'a> fmt::Display for TriggerType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(trigger) => write!(f, "{}", trigger),
            Self::Layer((mode, layer)) => write!(f, "{}[{}]", mode, format_indices(layer)),
            Self::Indicator(indicators) => {
                if indicators.len() > 1 {
                    write!(f, "I[{}]", format_indices(indicators))
                } else {
                    write!(f, "I{}", format_indices(indicators))
                }
            }
            Self::Generic((bank, index, param)) => {
                if let Some(param) = &param {
                    write!(f, "T[{}, {}]({})", bank, index, param)
                } else {
                    write!(f, "T[{}, {}]", bank, index)
                }
            }
            Self::Animation(name) => write!(f, "A[{}]", name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Trigger<'a> {
    pub trigger: TriggerType<'a>,
    pub state: Option<StateMap>,
}

impl<'a> fmt::Display for Trigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(state) = &self.state {
            write!(f, "{}({})", self.trigger, state)
        } else {
            write!(f, "{}", self.trigger)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
    pub kind: StateType,
    pub time: Option<usize>,
}

impl<'a> fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(time) = self.time {
            write!(f, "{}:{}", self.kind, time)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct StateMap {
    pub states: Vec<State>,
}

impl StateMap {
    pub fn from_map(map: Map) -> Self {
        let mut states = vec![];
        for (k, v) in map.iter() {
            let mut state = State {
                kind: StateType::from_str(k),
                time: None,
            };
            if let Ok(v) = v.parse::<usize>() {
                state.time = Some(v);
            }
            states.push(state);
        }

        StateMap { states }
    }
}

impl<'a> fmt::Display for StateMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.states
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum StateType {
    // Key
    Press,         // (P)
    Hold,          // (H)
    Release,       // (R)
    Unpressed,     // (O)
    UniquePress,   // (UP)
    UniqueRelease, // (UR)
    Analog(usize), // 0-100

    // Other
    Activate,   // (A)
    On,         // (On)
    Deactivate, // (D)
    Off,        // (Off)
}

impl StateType {
    pub fn from_str(s: &str) -> Self {
        match s {
            // Key
            "P" => Self::Press,
            "H" => Self::Hold,
            "R" => Self::Release,
            "O" => Self::Unpressed,
            "UP" => Self::UniquePress,
            "UR" => Self::UniqueRelease,

            // Other
            "A" => Self::Activate,
            "On" => Self::On,
            "D" => Self::Deactivate,
            "Off" => Self::Off,
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Key
            Self::Press => write!(f, "P"),
            Self::Hold => write!(f, "H"),
            Self::Release => write!(f, "R"),
            Self::Unpressed => write!(f, "O"),
            Self::UniquePress => write!(f, "UP"),
            Self::UniqueRelease => write!(f, "UR"),
            Self::Analog(v) => write!(f, "{}", v),

            // Other
            Self::Activate => write!(f, "A"),
            Self::On => write!(f, "On"),
            Self::Deactivate => write!(f, "D"),
            Self::Off => write!(f, "Off"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Key<'a> {
    Scancode(usize),
    Char(&'a str),
    Usb(&'a str),
    Consumer(&'a str),
    System(&'a str),
    Unicode(&'a str),
    None,
}

impl<'a> Key<'a> {
    pub fn value(&self, layout: &Layout) -> usize {
        use crate::parser::parse_int;
        match self {
            Key::Scancode(num) => *num,
            Key::Char(c) => parse_int(&layout.from_hid_keyboard[*c]),
            Key::Usb(name) => parse_int(&layout.from_hid_keyboard[*name]),
            Key::Consumer(name) => parse_int(&layout.from_hid_consumer[*name]),
            Key::System(name) => parse_int(&layout.from_hid_sysctrl[*name]),
            Key::Unicode(_) => 0, // xxx
            Key::None => 0,
        }
    }
}

impl<'a> fmt::Display for Key<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Scancode(num) => write!(f, "S{}", num),
            Key::Char(num) => write!(f, "'{}'", num),
            Key::Usb(name) => write!(f, "U{}", name),
            Key::Consumer(name) => write!(f, "CONS{}", name),
            Key::System(name) => write!(f, "SYS{}", name),
            Key::Unicode(name) => write!(f, "U+{}", name),
            Key::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ResultType<'a> {
    Output(Key<'a>),
    Layer((LayerMode, Indices)),
    Animation(AnimationResult<'a>),
    Pixel(Pixel<'a>),
    PixelLayer(Pixel<'a>),
    Capability((Capability<'a>, Option<StateMap>)),
    Text(&'a str),
    UnicodeText(&'a str),
    NOP,
}

impl<'a> fmt::Display for ResultType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Output(trigger) => write!(f, "{}", trigger),
            Self::Layer((mode, layers)) => write!(f, "{}[{}]", mode, format_indices(layers)),
            Self::Animation(trigger) => write!(f, "{}", trigger),
            Self::Pixel(trigger) => write!(f, "{}", trigger),
            Self::PixelLayer(trigger) => write!(f, "{}", trigger),
            Self::Capability((trigger, state)) => {
                if let Some(state) = state {
                    write!(f, "{}({})", trigger, state)
                } else {
                    write!(f, "{}", trigger)
                }
            }
            Self::Text(text) => write!(f, "\"{}\"", text),
            Self::UnicodeText(text) => write!(f, "u\"{}\"", text),
            Self::NOP => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action<'a> {
    pub result: ResultType<'a>,
    pub state: Option<StateMap>,
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(state) = &self.state {
            write!(f, "{}:{}", self.result, state)
        } else {
            write!(f, "{}", self.result)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerMode {
    Replace,            // :
    SoftReplace,        // ::
    Add,                // :+
    Remove,             // :-
    IsolateReplace,     // i:
    IsolateSoftReplace, // i::
    IsolateAdd,         // i:+
    IsolateRemove,      // i:-
}

impl TriggerMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            ":" => Self::Replace,
            "::" => Self::SoftReplace,
            ":+" => Self::Add,
            ":-" => Self::Remove,
            "i:" => Self::IsolateReplace,
            "i::" => Self::IsolateSoftReplace,
            "i:+" => Self::IsolateAdd,
            "i:-" => Self::IsolateRemove,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TriggerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replace => write!(f, ":"),
            Self::SoftReplace => write!(f, "::"),
            Self::Add => write!(f, ":+"),
            Self::Remove => write!(f, ":-"),
            Self::IsolateReplace => write!(f, "i:"),
            Self::IsolateSoftReplace => write!(f, "i::"),
            Self::IsolateAdd => write!(f, "i:+"),
            Self::IsolateRemove => write!(f, "i:-"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelAddr {
    Absolute(usize),
    RelativeInt(usize),
    RelativePercent(usize),
}

impl PixelAddr {
    pub fn from_str(s: &str) -> PixelAddr {
        PixelAddr::Absolute(s.parse::<usize>().unwrap_or(0)) // XXX
    }
}

impl fmt::Display for PixelAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absolute(v) => write!(f, "{}", v),
            Self::RelativeInt(v) => write!(f, "{:+}", v),
            Self::RelativePercent(v) => write!(f, "{:+}%", v),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PixelRange<'a> {
    pub index: Option<PixelAddr>,
    pub row: Option<PixelAddr>,
    pub col: Option<PixelAddr>,
    pub key: Option<Key<'a>>,
}

impl<'a> PixelRange<'a> {
    pub fn from_map(map: Map) -> Self {
        let mut pos = PixelRange::default();
        for (k, v) in map.iter() {
            match *k {
                "i" => pos.index = Some(PixelAddr::from_str(v)),
                "r" => pos.row = Some(PixelAddr::from_str(v)),
                "c" => pos.col = Some(PixelAddr::from_str(v)),
                _ => {}
            }
        }

        pos
    }
}

impl<'a> fmt::Display for PixelRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(index) = &self.index {
            write!(f, "{}", index)?;
        }
        if let Some(row) = &self.row {
            write!(f, "r:{}", row)?;
        }
        if let Some(col) = &self.col {
            write!(f, "c:{}", col)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AnimationResult<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> fmt::Display for AnimationResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A[{}]({})", self.name, self.args.join(", "))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Pixel<'a> {
    pub range: PixelRange<'a>,
    pub channel_values: Vec<PixelColor>,
}

impl<'a> fmt::Display for Pixel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P[{}]({})",
            self.range,
            self.channel_values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelColor {
    Rgb(usize),
    Relative(isize),
    RelativeNoRoll(isize),
    Shift(isize),
}

impl fmt::Display for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(v) => write!(f, "{}", v),
            Self::Relative(v) => write!(f, "{:+}", v),
            Self::RelativeNoRoll(v) => write!(f, ":{:+}", v),
            Self::Shift(v) => write!(f, "<{:+}", v),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct KllFile<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> fmt::Display for KllFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}
