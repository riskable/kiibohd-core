use pest_consume::{match_nodes, Error, Parser};
use std::collections::HashMap;

type PestError = Error<Rule>;
type Result<T> = std::result::Result<T, PestError>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

fn parse_int(s: &str) -> usize {
    //dbg!(s);
    if s.starts_with("0x") {
        usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
    } else {
        usize::from_str_radix(s, 10).unwrap()
    }
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
    Variable((Variable<'a>, &'a str)),
    Capability((&'a str, Capability<'a>)),
    Keymap((Trigger<'a>, TriggerVarient, Action<'a>)),
    Position((usize, Position)),
    Pixelmap((usize, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, usize, Vec<Pixel>)),
    NOP,
}

use std::fmt;
impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define((name, val)) => write!(f, "{} = {};", name, maybe_quote(val)),
            Self::Variable((var, val)) => match var {
                Variable::Array(name, index) => write!(
                    f,
                    "{}[{}] = {};",
                    maybe_quote(name),
                    index,
                    maybe_quote(val)
                ),
                Variable::String(name) => {
                    write!(f, "{} = {};", maybe_quote(name), maybe_quote(val))
                }
            },
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Variable<'a> {
    Array(&'a str, usize),
    String(&'a str),
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    x: usize,  // mm
    y: usize,  // mm
    z: usize,  // mm
    rx: usize, // deg
    ry: usize, // deg
    rz: usize, // deg
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
    channels: Vec<(usize, usize)>,
    scancode: Option<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Animation<'a> {
    modifiers: Vec<&'a str>,
    frames: Vec<Vec<Pixel>>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Capability<'a> {
    function: &'a str,
    args: Vec<&'a str>,
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
    keys: KeyGroup<'a>,
    press_state: Option<KeyState>,
    analog_state: Option<usize>, // percent (0-100)
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
    indicator: usize,
    state: Option<GenericState>,
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
    layer: usize,
    mode: LayerMode,
    state: Option<GenericState>,
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
    bank: usize,
    index: usize,
    param: Option<usize>,
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
    Other(&'a str),
}

impl<'a> fmt::Display for Trigger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(trigger) => write!(f, "{}", trigger),
            Self::Other(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Key<'a> {
    Scancode(usize),
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
            Key::Usb(name) => write!(f, "U{}", name),
            Key::Consumer(name) => write!(f, "CON{}", name),
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
    Capability((Capability<'a>, KeyState)),
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
            Self::Capability((trigger, state)) => write!(f, "{}({})", trigger, state),
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
    index: Option<PixelAddr>,
    row: Option<PixelAddr>,
    col: Option<PixelAddr>,
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
    name: &'a str,
    state: Option<GenericState>,
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
    name: &'a str,
    args: Vec<&'a str>,
}

impl<'a> fmt::Display for AnimationAction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A[{}]({})", self.name, self.args.join(", "))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Pixel {
    range: PixelRange,
    channel_values: Vec<PixelColor>,
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

#[pest_consume::parser]
impl KLLParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    fn file(input: Node) -> Result<Vec<Statement>> {
        Ok(match_nodes!(input.into_children();
            [statement(statements).., _] => statements.collect(),
        ))
    }

    fn word(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn string(input: Node) -> Result<&str> {
        Ok(input.as_str().trim_matches('"'))
    }
    fn number(input: Node) -> Result<usize> {
        Ok(parse_int(input.as_str()))
    }

    fn name(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn value(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
    fn array(input: Node) -> Result<(&str, usize)> {
        Ok(match_nodes!(input.into_children();
            [name(n), number(i)] => (n, i),
        ))
    }

    fn kv(input: Node) -> Result<(&str, &str)> {
        Ok(match_nodes!(input.into_children();
            [word(k), word(v)] => (k,v),
            [word(k), ] => (k,"")
        ))
    }
    fn kvmap(input: Node) -> Result<HashMap<&str, &str>> {
        Ok(match_nodes!(input.into_children();
            [kv(kv)..] => kv.collect(),
        ))
    }

    fn function(input: Node) -> Result<Capability> {
        Ok(match_nodes!(input.into_children();
            [name(n), kvmap(args)] => Capability {
                function: n,
                args: args.keys().map(|x| *x).collect(), // XXX
            }
        ))
    }

    fn binding(input: Node) -> Result<TriggerVarient> {
        Ok(match input.as_str() {
            ":" => TriggerVarient::Replace,
            "::" => TriggerVarient::SoftReplace,
            ":+" => TriggerVarient::Add,
            ":-" => TriggerVarient::Remove,
            "i:" => TriggerVarient::IsolateReplace,
            "i::" => TriggerVarient::IsolateSoftReplace,
            "i:+" => TriggerVarient::IsolateAdd,
            "i:-" => TriggerVarient::IsolateRemove,
            _ => unreachable!(),
        })
    }

    fn channel(input: Node) -> Result<PixelColor> {
        let color = input.as_str();
        Ok(match &color[0..1] {
            "+" | "-" => PixelColor::Relative(color.parse::<isize>().unwrap()),
            _ => PixelColor::Rgb(parse_int(color)),
        })
    }

    fn pixelval(input: Node) -> Result<Pixel> {
        Ok(match_nodes!(input.into_children();
            [kvmap(index), channel(c)..] => {
                Pixel {
                    range: PixelRange {
                        index: Some(PixelAddr::Absolute(parse_int(index.keys().next().unwrap()))), // XXX
                        row: None,
                        col: None,
                    },
                    channel_values: c.collect(),
                }
            }
        ))
    }

    fn scancode(input: Node) -> Result<usize> {
        Ok(parse_int(input.as_str().strip_prefix("S").unwrap()))
    }
    fn usbcode(input: Node) -> Result<Key> {
        let usbcode = input.as_str().strip_prefix("U").unwrap();
        Ok(Key::Usb(usbcode))
    }
    fn consumer(input: Node) -> Result<Key> {
        let concode = input.as_str().strip_prefix("CON").unwrap();
        Ok(Key::Consumer(concode))
    }
    fn system(input: Node) -> Result<Key> {
        let syscode = input.as_str().strip_prefix("SYS").unwrap();
        Ok(Key::System(syscode))
    }
    fn none(_input: Node) -> Result<&str> {
        Ok("None")
    }
    fn key(input: Node) -> Result<Key> {
        Ok(match_nodes!(input.into_children();
                [scancode(s)] => Key::Scancode(s),
                [usbcode(key)] => key,
                [consumer(key)] => key,
                [system(key)] => key,
                [none(_)] => Key::None,
        ))
    }

    fn key_trigger(input: Node) -> Result<KeyTrigger> {
        let trigger = match_nodes!(input.into_children();
            [key(key), kvmap(state)] => KeyTrigger {
                keys: KeyGroup::Single(key), // XXX
                press_state: None, // XXX
                analog_state: None, // XXX
            },
            [key(key)] => KeyTrigger {
                keys: KeyGroup::Single(key),
                press_state: None,
                analog_state: None,
            },

        );
        Ok(trigger)
    }
    fn trigger(input: Node) -> Result<Trigger> {
        Ok(match_nodes!(input.into_children();
            [key_trigger(trigger)] => Trigger::Key(trigger),
        ))
    }

    fn result(input: Node) -> Result<Action> {
        Ok(match_nodes!(input.into_children();
            [usbcode(key)] => Action::Output(KeyTrigger {
                keys: KeyGroup::Single(key),
                press_state: None,
                analog_state: None,
            }),
            [consumer(key)] => Action::Output(KeyTrigger {
                keys: KeyGroup::Single(key),
                press_state: None,
                analog_state: None,
            }),
            [system(key)] => Action::Output(KeyTrigger {
                keys: KeyGroup::Single(key),
                press_state: None,
                analog_state: None,
            }),
            [pixelval(pixel)] => Action::Pixel(pixel),
            [none(_)] => Action::NOP,
        ))
    }

    fn property(input: Node) -> Result<Statement> {
        let (variable, value) = match_nodes!(input.into_children();
            [array((n,i)), value(v)] => (Variable::Array(n, i), v),
            [string(n), value(v)] => (Variable::String(n), v),
        );
        Ok(Statement::Variable((variable, value)))
    }
    fn define(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(n), value(v)] => Statement::Define((n, v))
        ))
    }
    fn capability(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(n), function(f)] =>  Statement::Capability((n, f)),
        ))
    }
    fn mapping(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [trigger(trigger), binding(mode), result(result)] => Statement::Keymap((trigger, mode, result)),
        ))
    }
    fn position(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [number(index), kvmap(map)] => {
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

                Statement::Position((index, pos))
            }
        ))
    }
    fn pixelmap(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [number(index), kvmap(channelmap), scancode(scancode)] => {
                let channels = channelmap
                    .iter()
                    .map(|(k, v)| {
                        let k = k.parse::<usize>().unwrap();
                        let v = v.parse::<usize>().unwrap();
                        (k, v)
                    })
                    .collect::<Vec<_>>();
                let pixel = PixelDef {
                    scancode: Some(scancode),
                    channels,
                };
                Statement::Pixelmap((index, pixel))
            }
        ))
    }
    fn animdef(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(name), kvmap(args)] => {
                let animation = Animation {
                    modifiers: args.keys().map(|x| *x).collect(), // XXX
                    frames: vec![],
                };

                Statement::Animation((name, animation))
            }
        ))
    }
    fn animframe(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [name(name), number(index), pixelval(pixels)..] => {
                Statement::Frame((name, index, pixels.collect()))
            }
        ))
    }
    fn statement(input: Node) -> Result<Statement> {
        //dbg!(input.as_str());
        Ok(match_nodes!(input.into_children();
                [property(stmt)] => stmt,
                [define(stmt)] => stmt,
                [capability(stmt)] => stmt,
                [mapping(stmt)]=> stmt,
                [position(stmt)] => stmt,
                [pixelmap(stmt)] => stmt,
                [animdef(stmt)] => stmt,
                [animframe(stmt)] => stmt,
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct KllFile<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Default, Clone)]
pub struct KllState<'a> {
    defines: HashMap<&'a str, &'a str>,
    variables: HashMap<Variable<'a>, &'a str>,
    capabilities: HashMap<&'a str, Capability<'a>>,
    keymap: Vec<(Trigger<'a>, TriggerVarient, Action<'a>)>,
    positions: HashMap<usize, Position>,
    pixelmap: HashMap<usize, PixelDef>,
    animations: HashMap<&'a str, Animation<'a>>,
}

impl<'a> fmt::Display for KllFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl<'a> KllFile<'a> {
    fn from_str(text: &str) -> Result<KllFile> {
        let inputs = KLLParser::parse(Rule::file, text)?;
        let input = inputs.single()?;

        let kll = KllFile {
            statements: KLLParser::file(input)?,
        };

        Ok(kll)
    }

    pub fn into_struct(self) -> KllState<'a> {
        let mut kll = KllState::default();
        for statement in self.statements {
            match statement {
                Statement::Define((name, val)) => {
                    kll.defines.insert(name, val);
                }
                Statement::Variable((name, val)) => {
                    kll.variables.insert(name, val);
                }
                Statement::Capability((name, cap)) => {
                    kll.capabilities.insert(name, cap);
                }
                Statement::Keymap((trigger, varient, action)) => {
                    kll.keymap.push((trigger, varient, action));
                }
                Statement::Position((index, pos)) => {
                    kll.positions.insert(index, pos);
                }
                Statement::Pixelmap((index, map)) => {
                    kll.pixelmap.insert(index, map);
                }
                Statement::Animation((name, anim)) => {
                    kll.animations.insert(name, anim);
                }
                Statement::Frame((name, index, frame)) => {
                    let animation = kll.animations.entry(name).or_default();
                    let frames = &mut animation.frames;
                    if frames.len() <= index {
                        frames.resize(index + 1, vec![]);
                    }
                    frames[index] = frame;
                }
                Statement::NOP => {}
            };
        }

        kll
    }
}

pub fn parse(text: &str) -> Result<KllFile> {
    KllFile::from_str(text)
}

#[cfg(test)]
mod tests {
    use crate::pest::Parser;
    use crate::{KLLParser, Rule};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        /*let successful_parse = KLLParser::parse(Rule::field, "-273.15");
        println!("{:?}", successful_parse);

        let unsuccessful_parse = KLLParser::parse(Rule::field, "this is not a number");
        println!("{:?}", unsuccessful_parse);*/
    }
}
