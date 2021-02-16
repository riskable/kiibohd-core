use std::fmt;

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
    Keymap((Trigger<'a>, TriggerVarient, Action<'a>)),
    Position((usize, Position)),
    Pixelmap((usize, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, usize, Vec<Pixel>)),
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
            Self::Keymap((trigger, varient, action)) => {
                write!(f, "{} {} {};", trigger, varient, action)
            }
            Self::Position((index, pos)) => write!(f, "P[{}] <= {};", index, pos),
            Self::Pixelmap((index, map)) => write!(
                f,
                "P[{}]{} : {};",
                index,
                map.channels
                    .iter()
                    .map(|(c, w)| format!("{}:{}", c, w))
                    .collect::<Vec<String>>()
                    .join(", "),
                map.scancode
                    .map(|x| format!("S{}", x))
                    .unwrap_or_else(|| "None".to_string())
            ),
            Self::Animation((name, anim)) => {
                write!(f, "A[{}] <= {};", name, anim.modifiers.join(", "))
            }
            Self::Frame((name, index, frame)) => {
                write!(f, "A[{}, {}] <= {:?};", name, index, frame)
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

#[derive(Debug, Default, Clone)]
pub struct Animation<'a> {
    pub modifiers: Vec<&'a str>,
    pub frames: Vec<Vec<Pixel>>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Capability<'a> {
    pub function: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> fmt::Display for Capability<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.function, self.args.join(", "))
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
    pub indicator: usize,
    pub state: Option<GenericState>,
}

impl<'a> fmt::Display for IndicatorTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I{}", self.indicator)?;
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
    pub layer: usize,
    pub mode: LayerMode,
    pub state: Option<GenericState>,
}

impl<'a> fmt::Display for LayerTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.mode, self.layer)?;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
}

impl fmt::Display for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(v) => write!(f, "{}", v),
            Self::Relative(v) => write!(f, "{:+}", v),
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
