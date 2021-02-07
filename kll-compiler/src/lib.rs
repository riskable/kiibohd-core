use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

type PestError = pest::error::Error<Rule>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

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
    Variable((&'a str, Variable<'a>)),
    Capability((&'a str, Capability<'a>)),
    Keymap((Trigger<'a>, TriggerVarient, Action<'a>)),
    Position((usize, Position)),
    Pixelmap((usize, PixelDef)),
    Animation((&'a str, Animation<'a>)),
    Frame((&'a str, usize, Pixel)),
    NOP,
}

use std::fmt;
impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define((name, val)) => write!(f, "{} = {};", name, maybe_quote(val)),
            Self::Variable((name, val)) => match val {
                Variable::Array(index, val) => write!(
                    f,
                    "{}[{}] = {};",
                    maybe_quote(name),
                    index,
                    maybe_quote(val)
                ),
                Variable::String(val) => write!(f, "{} = {};", maybe_quote(name), maybe_quote(val)),
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
            Self::Frame((name, index, frame)) => write!(f, "A[{}, {}] <= {};", name, index, frame),
            Self::NOP => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Variable<'a> {
    Array(usize, &'a str),
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
    frames: Vec<Pixel>,
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

#[derive(Debug, Default, Clone)]
pub struct KllFile<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Default, Clone)]
pub struct KllState<'a> {
    defines: HashMap<&'a str, &'a str>,
    variables: HashMap<&'a str, Variable<'a>>,
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

fn parse_int(s: &str) -> usize {
    //dbg!(s);
    if s.starts_with("0x") {
        usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
    } else {
        usize::from_str_radix(s, 10).unwrap()
    }
}

fn parse_array<'a>(lhs: Pair<'a, Rule>, rhs: &'a str) -> Statement<'a> {
    let mut inner = lhs.into_inner();
    let name = inner.next().unwrap().as_str();
    let index = inner.next().unwrap().as_str().parse::<usize>().unwrap();
    let value = Variable::Array(index, rhs);
    Statement::Variable((name, value))
}

fn parse_variable<'a>(lhs: Pair<'a, Rule>, rhs: &'a str) -> Statement<'a> {
    let name = lhs.as_str().trim_matches('"');
    let value = Variable::String(rhs);
    Statement::Variable((name, value))
}

fn parse_property(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap().as_str().trim_matches('"');

    match lhs.as_rule() {
        Rule::array => parse_array(lhs, rhs),
        Rule::string => parse_variable(lhs, rhs),
        _ => unreachable!(),
    }
}

fn parse_define(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let name = parts.next().unwrap().as_str();
    let value = parts.next().unwrap().as_str();
    Statement::Define((name, value))
}

fn parse_fn(element: Pair<Rule>) -> Capability {
    let mut parts = element.into_inner();
    let fun = parts.next().unwrap();
    let args = parts.next().unwrap();

    Capability {
        function: fun.as_str(),
        args: parse_args(args),
    }
}

fn parse_args(element: Pair<Rule>) -> Vec<&str> {
    let args = element.into_inner();

    let mut result = vec![];
    for item in args {
        result.push(item.as_str());
    }
    result
}

fn parse_capability(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let name = parts.next().unwrap().as_str();
    let rhs = parts.next().unwrap();

    let cap = parse_fn(rhs);
    Statement::Capability((name, cap))
}

fn parse_mode(mode: Pair<Rule>) -> TriggerVarient {
    match mode.as_str() {
        ":" => TriggerVarient::Replace,
        "::" => TriggerVarient::SoftReplace,
        ":+" => TriggerVarient::Add,
        ":-" => TriggerVarient::Remove,
        "i:" => TriggerVarient::IsolateReplace,
        "i::" => TriggerVarient::IsolateSoftReplace,
        "i:+" => TriggerVarient::IsolateAdd,
        "i:-" => TriggerVarient::IsolateRemove,
        _ => unreachable!(),
    }
}

fn parse_trigger(lhs: Pair<Rule>) -> Trigger {
    let text = lhs.as_str();
    let trigger = lhs.into_inner().next().unwrap();
    match trigger.as_rule() {
        Rule::key_trigger => {
            let scancode = text.strip_prefix("S").unwrap();
            let key = Key::Scancode(parse_int(scancode));
            let trigger = KeyTrigger {
                keys: KeyGroup::Single(key),
                press_state: None,
                analog_state: None,
            };
            Trigger::Key(trigger)
        }
        _ => unimplemented!(),
    }
}

fn parse_scancode(text: &str) -> usize {
    text.strip_prefix("S").unwrap().parse::<usize>().unwrap()
}

fn parse_usb(text: &str) -> KeyTrigger {
    let usbcode = text.strip_prefix("U").unwrap();
    let key = Key::Usb(usbcode);
    KeyTrigger {
        keys: KeyGroup::Single(key),
        press_state: None,
        analog_state: None,
    }
}

fn parse_consumer(text: &str) -> KeyTrigger {
    let code = text.strip_prefix("CON").unwrap();
    let key = Key::Consumer(code);
    KeyTrigger {
        keys: KeyGroup::Single(key),
        press_state: None,
        analog_state: None,
    }
}

fn parse_system(text: &str) -> KeyTrigger {
    let code = text.strip_prefix("Sys").unwrap();
    let key = Key::System(code);
    KeyTrigger {
        keys: KeyGroup::Single(key),
        press_state: None,
        analog_state: None,
    }
}

fn parse_channels(channels: Pair<Rule>) -> Vec<PixelColor> {
    let mut values = vec![];
    for c in channels.into_inner() {
        let color = c.as_str();
        let value = match &color[0..1] {
            "+" | "-" => PixelColor::Relative(color.parse::<isize>().unwrap()),
            _ => PixelColor::Rgb(parse_int(color)),
        };
        values.push(value);
    }
    values
}

fn parse_pixel(element: Pair<Rule>) -> Pixel {
    let mut parts = element.into_inner();
    let index = parts.next().unwrap().as_str();
    let channels = parts.next().unwrap();

    Pixel {
        range: PixelRange {
            index: Some(PixelAddr::Absolute(parse_int(index))),
            row: None,
            col: None,
        },
        channel_values: parse_channels(channels),
    }
}

fn parse_result(rhs: Pair<Rule>) -> Action {
    let text = rhs.as_str();
    let result = rhs.into_inner().next().unwrap();
    match dbg!(result.as_rule()) {
        Rule::usbcode => Action::Output(parse_usb(text)),
        Rule::consumer => Action::Output(parse_consumer(text)),
        Rule::system => Action::Output(parse_system(text)),
        Rule::color => Action::Pixel(parse_pixel(result)),
        _ => unimplemented!(),
    }
}

fn parse_mapping(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let lhs = parts.next().unwrap();
    let assignment = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let trigger = parse_trigger(lhs);
    let mode = parse_mode(assignment);
    let result = parse_result(rhs);

    Statement::Keymap((trigger, mode, result))
}

fn parse_kv(kv: Pair<Rule>) -> (&str, &str) {
    let mut parts = kv.into_inner();
    let k = parts.next().unwrap().as_str();
    let v = parts.next().unwrap().as_str();
    (k, v)
}

fn parse_kvmap(element: Pair<Rule>) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for kv in element.into_inner() {
        let (k, v) = parse_kv(kv);
        map.insert(k, v);
    }

    map
}

fn parse_list(element: Pair<Rule>) -> Vec<&str> {
    let mut list = vec![];
    for item in element.into_inner() {
        list.push(item.as_str());
    }

    list
}

fn parse_position(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let index = parts.next().unwrap().as_str();
    let map = parse_kvmap(parts.next().unwrap());

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

    Statement::Position((parse_int(index), pos))
}

fn parse_pixelmap_left(lhs: Pair<Rule>) -> (usize, Vec<(usize, usize)>) {
    let mut lhs = lhs.into_inner();
    let index = parse_int(lhs.next().unwrap().as_str());

    let channelmap = parse_kvmap(lhs.next().unwrap());
    let channels = channelmap
        .iter()
        .map(|(k, v)| {
            let k = k.parse::<usize>().unwrap();
            let v = v.parse::<usize>().unwrap();
            (k, v)
        })
        .collect::<Vec<_>>();

    (index, channels)
}

fn parse_pixelmap(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let (index, channels) = parse_pixelmap_left(lhs);
    let scancode = parse_scancode(rhs.as_str());

    let pixel = PixelDef {
        scancode: Some(scancode),
        channels,
    };

    Statement::Pixelmap((index, pixel))
}

fn parse_str(element: Pair<Rule>) -> &str {
    let mut element = element.into_inner();
    element.next().unwrap().as_str()
}

fn parse_animdef(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let name = parse_str(lhs);
    let animation = Animation {
        modifiers: parse_list(rhs),
        frames: vec![],
    };

    Statement::Animation((name, animation))
}

fn parse_anim_left(element: Pair<Rule>) -> (&str, usize) {
    let mut parts = element.into_inner();
    let name = parts.next().unwrap().as_str();
    let index = parts.next().unwrap().as_str().parse::<usize>().unwrap();
    (name, index)
}

fn parse_anim_right(elements: Pair<Rule>) -> Pixel {
    let mut parts = elements.into_inner();
    let pixel = parts.next().unwrap();
    parse_pixel(pixel)
}

fn parse_animframe(line: Pair<Rule>) -> Statement {
    let mut parts = line.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let (name, index) = parse_anim_left(lhs);
    let pixel = parse_anim_right(rhs);
    Statement::Frame((name, index, pixel))
}

fn parse_statement(line: Pair<Rule>) -> Statement {
    match line.as_rule() {
        Rule::property => parse_property(line),
        Rule::define => parse_define(line),
        Rule::capability => parse_capability(line),
        Rule::mapping => parse_mapping(line),
        Rule::position => parse_position(line),
        Rule::pixelmap => parse_pixelmap(line),
        Rule::animdef => parse_animdef(line),
        Rule::animframe => parse_animframe(line),
        Rule::EOI => Statement::NOP,
        _ => unreachable!(),
    }
}

impl<'a> KllFile<'a> {
    fn from_str(text: &str) -> Result<KllFile, PestError> {
        let mut kll = KllFile::default();

        let file = KLLParser::parse(Rule::file, text)?;
        for line in file {
            kll.statements.push(parse_statement(line));
        }

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
                        frames.resize(index + 1, Pixel::default());
                    }
                    frames[index] = frame;
                }
                Statement::NOP => {}
            };
        }

        kll
    }
}

pub fn parse(text: &str) -> Result<KllFile, PestError> {
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
