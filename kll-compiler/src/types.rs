use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

pub type Indices = Vec<Range<usize>>;
pub type Map<'a> = HashMap<&'a str, &'a str>;

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
    Keymap((Vec<Trigger<'a>>, TriggerVarient, Vec<Action<'a>>)),
    Position((Indices, Position)),
    Pixelmap((Indices, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, Indices, Vec<Pixel>)),
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
            Self::Position((indices, pos)) => write!(f, "P[{:?}] <= {};", indices, pos),
            Self::Pixelmap((indices, map)) => write!(
                f,
                "P[{:?}]{} : {};",
                indices,
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
            Self::Frame((name, indices, frame)) => {
                write!(f, "A[{}, {:?}] <= {:?};", name, indices, frame)
            }
            Self::NOP => Ok(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub x: usize,  // mm
    pub y: usize,  // mm
    pub z: usize,  // mm
    pub rx: usize, // deg
    pub ry: usize, // deg
    pub rz: usize, // deg
}

impl Position {
    pub fn from_map(map: Map) -> Self {
        let mut pos = Position::default();
        for (k, v) in map.iter() {
            let v = v.parse::<usize>().unwrap();
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
        if self.x != 0 {
            write!(f, "x:{}", self.x)?;
        }
        if self.y != 0 {
            write!(f, "y:{}", self.y)?;
        }
        if self.z != 0 {
            write!(f, "z:{}", self.z)?;
        }
        if self.rx != 0 {
            write!(f, "x:{}", self.rx)?;
        }
        if self.ry != 0 {
            write!(f, "y:{}", self.ry)?;
        }
        if self.rz != 0 {
            write!(f, "z:{}", self.rz)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct PixelDef {
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
    pub frames: Vec<Vec<Pixel>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Capability<'a> {
    pub function: &'a str,
    pub args: Map<'a>,
}

impl<'a> fmt::Display for Capability<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", self.function, self.args)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyState {
    Press(usize),   // (P:time)
    Hold(usize),    // (H:time)
    Release(usize), // (R:time)
    Off,            // (O).  Not available for output
    UniquePress,    // (UP). Not available for output
    UniqueRelease,  // (UR). Not available for output
}

impl KeyState {
    pub fn from_str(s: &str) -> Self {
        match s {
            "P" => Self::Press(0),
            "H" => Self::Hold(0),
            "R" => Self::Release(0),
            "O" => Self::Off,
            "UP" => Self::UniquePress,
            "UR" => Self::UniqueRelease,
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for KeyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Press(time) => {
                write!(f, "P")?;
                if *time != 0 {
                    write!(f, ":{}", time)?;
                }
            }
            Self::Hold(time) => {
                write!(f, "H")?;
                if *time != 0 {
                    write!(f, ":{}", time)?;
                }
            }
            Self::Release(time) => {
                write!(f, "R")?;
                if *time != 0 {
                    write!(f, ":{}", time)?;
                }
            }
            Self::Off => write!(f, "O")?,
            Self::UniquePress => write!(f, "UP")?,
            Self::UniqueRelease => write!(f, "UR")?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyGroup<'a> {
    Single(Key<'a>),
    Sequence(Vec<KeyGroup<'a>>),
    Combination(Vec<KeyGroup<'a>>),
}

impl<'a> fmt::Display for KeyGroup<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(key) => write!(f, "{}", key),
            Self::Sequence(sequence) => write!(
                f,
                "{}",
                sequence
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Combination(combo) => write!(
                f,
                "{}",
                combo
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" + ")
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeyTrigger<'a> {
    pub keys: KeyGroup<'a>,
    pub press_state: Option<KeyState>,
    pub analog_state: Option<usize>, // percent (0-100)
}

impl<'a> fmt::Display for KeyTrigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.keys)?;
        if let Some(press_state) = &self.press_state {
            write!(f, "{}", press_state)?;
        }
        if let Some(analog_state) = &self.analog_state {
            write!(f, "{}", analog_state)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GenericState {
    Activate,   // (A)
    On,         // (On)
    Deactivate, // (D)
    Off,        // (Off)
}

impl GenericState {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Activate,
            "On" => Self::On,
            "D" => Self::Deactivate,
            "Off" => Self::Off,
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for GenericState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Activate => write!(f, "A"),
            Self::On => write!(f, "On"),
            Self::Deactivate => write!(f, "D"),
            Self::Off => write!(f, "Off"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IndicatorTrigger {
    pub indicator: Indices,
    pub state: Option<GenericState>,
}

impl<'a> fmt::Display for IndicatorTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I{:?}", self.indicator)?;
        if let Some(state) = &self.state {
            write!(f, "{}", state)?;
        }
        Ok(())
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
pub struct LayerTrigger {
    pub layer: Indices,
    pub mode: LayerMode,
    pub state: Option<GenericState>,
}

impl<'a> fmt::Display for LayerTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{:?}]", self.mode, self.layer)?;
        if let Some(state) = &self.state {
            write!(f, "({})", state)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericTrigger {
    pub bank: usize,
    pub index: usize,
    pub param: Option<usize>,
}

impl<'a> fmt::Display for GenericTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T[{}, {}]", self.bank, self.index)?;
        if let Some(param) = &self.param {
            write!(f, "({})", param)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Trigger<'a> {
    Key(KeyTrigger<'a>),
    Layer(LayerTrigger),
    Indicator(IndicatorTrigger),
    Generic(GenericTrigger),
    Animation(&'a str),
    Other(&'a str),
}

impl<'a> fmt::Display for Trigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(trigger) => write!(f, "{}", trigger),
            Self::Layer(trigger) => write!(f, "{}", trigger),
            Self::Indicator(trigger) => write!(f, "{}", trigger),
            Self::Generic(trigger) => write!(f, "{}", trigger),
            Self::Animation(name) => write!(f, "A[{}]", name),
            Self::Other(text) => write!(f, "{}", text),
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
    Other(&'a str),
    None,
}

impl<'a> Key<'a> {
    pub fn value(&self) -> usize {
        // TODO: Add lookup tables
        0
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
            Key::Other(name) => write!(f, "{}", name),
            Key::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action<'a> {
    Output(KeyTrigger<'a>),
    Layer(LayerTrigger),
    Animation(AnimationAction<'a>),
    Pixel(Pixel),
    PixelLayer(Pixel),
    Capability((Capability<'a>, Option<KeyState>)),
    Other(&'a str),
    NOP,
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Output(trigger) => write!(f, "{}", trigger),
            Self::Layer(trigger) => write!(f, "{}", trigger),
            Self::Animation(trigger) => write!(f, "{}", trigger),
            Self::Pixel(trigger) => write!(f, "{}", trigger),
            Self::PixelLayer(trigger) => write!(f, "{}", trigger),
            Self::Capability((trigger, state)) => write!(f, "{}({:?})", trigger, state),
            Self::Other(trigger) => write!(f, "{}", trigger),
            Self::NOP => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerVarient {
    Replace,            // :
    SoftReplace,        // ::
    Add,                // :+
    Remove,             // :-
    IsolateReplace,     // i:
    IsolateSoftReplace, // i::
    IsolateAdd,         // i:+
    IsolateRemove,      // i:-
}

impl TriggerVarient {
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

impl fmt::Display for TriggerVarient {
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
pub struct PixelRange {
    pub index: Option<PixelAddr>,
    pub row: Option<PixelAddr>,
    pub col: Option<PixelAddr>,
    pub scancode: Option<usize>,
    pub usbcode: Option<usize>,
}

impl PixelRange {
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

impl fmt::Display for PixelRange {
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
pub struct AnimationTrigger<'a> {
    pub name: &'a str,
    pub state: Option<GenericState>,
}

impl<'a> fmt::Display for AnimationTrigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A[{}]", self.name)?;
        if let Some(state) = &self.state {
            write!(f, "({})", state)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AnimationAction<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> fmt::Display for AnimationAction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A[{}]({})", self.name, self.args.join(", "))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Pixel {
    pub range: PixelRange,
    pub channel_values: Vec<PixelColor>,
}

impl fmt::Display for Pixel {
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
