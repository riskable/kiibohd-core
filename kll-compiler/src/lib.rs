use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

type PestError = pest::error::Error<Rule>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

#[derive(Debug, Clone)]
pub enum Variable<'a> {
    List(Vec<&'a str>),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyState {
    Press(usize),   // (P:time)
    Hold(usize),    // (H:time)
    Release(usize), // (R:time)
    Off,            // (O).  Not available for output
    UniquePress,    // (UP). Not available for output
    UniqueRelease,  // (UR). Not available for output
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyGroup<'a> {
    Single(Key<'a>),
    Sequence(Vec<KeyGroup<'a>>),
    Combination(Vec<KeyGroup<'a>>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeyTrigger<'a> {
    keys: KeyGroup<'a>,
    press_state: Option<KeyState>,
    analog_state: Option<usize>, // percent (0-100)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GenericState {
    Activate,   // (A)
    On,         // (On)
    Deactivate, // (D)
    Off,        // (Off)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IndicatorTrigger {
    indicator: usize,
    state: Option<GenericState>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LayerMode {
    Normal,
    Shift,
    Latch,
    Lock,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LayerTrigger {
    layer: usize,
    mode: LayerMode,
    state: Option<GenericState>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericTrigger {
    bank: usize,
    index: usize,
    param: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Trigger<'a> {
    Key(KeyTrigger<'a>),
    Other(&'a str),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TriggerVarient {
    Replace,     // :
    SoftReplace, // ::
    Add,         // :+
    Remove,      // :-
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelAddr {
    Absolute(usize),
    RelativeInt(usize),
    RelativePercent(usize),
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PixelRange {
    index: Option<PixelAddr>,
    row: Option<PixelAddr>,
    col: Option<PixelAddr>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AnimationTrigger<'a> {
    name: &'a str,
    state: GenericState,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AnimationAction<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Pixel {
    range: PixelRange,
    channel_values: Vec<PixelColor>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PixelColor {
    Rgb(usize),
    Relative(isize),
}

#[derive(Debug, Clone)]
pub struct Mapping<'a> {
    trigger: Trigger<'a>,
    mode: TriggerVarient,
    isolate: bool,
    result: Action<'a>,
}

#[derive(Debug, Default, Clone)]
pub struct KllState<'a> {
    defines: HashMap<&'a str, &'a str>,
    variables: HashMap<&'a str, Variable<'a>>,
    capabilities: HashMap<&'a str, Capability<'a>>,
    keymap: Vec<Mapping<'a>>,
    positions: HashMap<usize, Position>,
    pixelmap: HashMap<usize, PixelDef>,
    animations: HashMap<&'a str, Animation<'a>>,
}

fn parse_int(s: &str) -> usize {
    //dbg!(s);
    if s.starts_with("0x") {
        usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
    } else {
        usize::from_str_radix(s, 10).unwrap()
    }
}

impl KllState<'_> {
    fn from_str(text: &str) -> Result<KllState, PestError> {
        let file = KLLParser::parse(Rule::file, text)?;

        let mut kll = KllState::default();

        for line in file {
            match line.as_rule() {
                Rule::property => {
                    let mut parts = line.into_inner();
                    let lhs = parts.next().unwrap();
                    let value = parts.next().unwrap().as_str().trim_matches('"');
                    //println!("SET '{}' to '{}'", lhs, value);

                    match lhs.as_rule() {
                        Rule::array => {
                            let mut inner = lhs.into_inner();
                            let name = inner.next().unwrap().as_str();
                            let index = inner.next().unwrap().as_str().parse::<usize>().unwrap();
                            let var = kll
                                .variables
                                .entry(name)
                                .or_insert_with(|| Variable::List(vec![]));
                            if let Variable::List(list) = var {
                                if list.len() <= index {
                                    list.resize(index + 1, "");
                                }
                                list[index] = value;
                            }
                        }
                        Rule::string => {
                            let name = lhs.as_str().trim_matches('"');
                            let value = Variable::String(value);
                            kll.variables.insert(name, value);
                        }
                        _ => unreachable!(),
                    };
                }
                Rule::define => {
                    let mut parts = line.into_inner();
                    let name = parts.next().unwrap().as_str();
                    let value = parts.next().unwrap().as_str();
                    //println!("DEFINE '{}' to '{}'", name, value);
                    kll.defines.insert(name, value);
                }
                Rule::capability => {
                    let mut parts = line.into_inner();
                    let name = parts.next().unwrap().as_str();
                    let mut rhs = parts.next().unwrap().into_inner();

                    let mut cap = Capability {
                        function: rhs.next().unwrap().as_str(),
                        ..Default::default()
                    };

                    let args = rhs.next().unwrap().into_inner();
                    for item in args {
                        cap.args.push(item.as_str());
                    }

                    //println!("CAP '{}' -> '{}'", name, value);
                    kll.capabilities.insert(name, cap);
                }
                Rule::mapping => {
                    let mut parts = line.into_inner();
                    let lhs = parts.next().unwrap();
                    let assignment = parts.next().unwrap().as_str();
                    let rhs = parts.next().unwrap();

                    let (isolate, mode) = match assignment {
                        ":" => (false, TriggerVarient::Replace),
                        "::" => (false, TriggerVarient::SoftReplace),
                        ":+" => (false, TriggerVarient::Add),
                        ":-" => (false, TriggerVarient::Remove),
                        "i:" => (true, TriggerVarient::Replace),
                        "i::" => (true, TriggerVarient::SoftReplace),
                        "i:+" => (true, TriggerVarient::Add),
                        "i:-" => (true, TriggerVarient::Remove),
                        _ => unreachable!(),
                    };

                    let text = lhs.as_str();
                    let trigger = match lhs.into_inner().next().unwrap().as_rule() {
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
                    };

                    let text = rhs.as_str();
                    let rhs = rhs.into_inner().next().unwrap();
                    let result = match dbg!(rhs.as_rule()) {
                        Rule::usbcode => {
                            let usbcode = text.strip_prefix("U").unwrap();
                            let key = Key::Usb(usbcode);
                            let trigger = KeyTrigger {
                                keys: KeyGroup::Single(key),
                                press_state: None,
                                analog_state: None,
                            };
                            Action::Output(trigger)
                        }
                        Rule::consumer => {
                            let code = text.strip_prefix("CON").unwrap();
                            let key = Key::Consumer(code);
                            let trigger = KeyTrigger {
                                keys: KeyGroup::Single(key),
                                press_state: None,
                                analog_state: None,
                            };
                            Action::Output(trigger)
                        }
                        Rule::system => {
                            let code = text.strip_prefix("Sys").unwrap();
                            let key = Key::System(code);
                            let trigger = KeyTrigger {
                                keys: KeyGroup::Single(key),
                                press_state: None,
                                analog_state: None,
                            };
                            Action::Output(trigger)
                        }
                        Rule::color => {
                            let mut parts = rhs.into_inner();
                            let index = parts.next().unwrap().as_str();
                            let channels = parts.next().unwrap();

                            let mut values = vec![];
                            for c in channels.into_inner() {
                                let color = c.as_str();
                                let value = match &color[0..1] {
                                    "+" | "-" => {
                                        PixelColor::Relative(color.parse::<isize>().unwrap())
                                    }
                                    _ => PixelColor::Rgb(parse_int(color)),
                                };
                                values.push(value);
                            }
                            let pixel = Pixel {
                                range: PixelRange {
                                    index: Some(PixelAddr::Absolute(parse_int(index))),
                                    ..Default::default()
                                },
                                channel_values: values,
                            };

                            Action::Pixel(pixel)
                        }
                        _ => unimplemented!(),
                    };

                    kll.keymap.push(Mapping {
                        trigger,
                        mode,
                        isolate,
                        result,
                    });
                }
                Rule::position => {
                    let mut parts = line.into_inner();
                    let index = parts.next().unwrap().as_str();

                    let mut pos = Position::default();
                    for kv in parts.next().unwrap().into_inner() {
                        let mut parts = kv.into_inner();
                        let k = parts.next().unwrap().as_str();
                        let v = parts.next().unwrap().as_str().parse::<usize>().unwrap();
                        match k {
                            "x" => pos.x = v,
                            "y" => pos.y = v,
                            "z" => pos.z = v,
                            "rx" => pos.rx = v,
                            "ry" => pos.ry = v,
                            "rz" => pos.rz = v,
                            _ => {}
                        }
                    }
                    //println!("POS '{}' -> '{}'", name, value);
                    kll.positions.insert(parse_int(index), pos);
                }
                Rule::pixelmap => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let scancode = parts.next().unwrap().as_str();

                    let index = parse_int(lhs.next().unwrap().as_str());
                    let mut pixel = PixelDef {
                        scancode: Some(
                            scancode
                                .strip_prefix("S")
                                .unwrap()
                                .parse::<usize>()
                                .unwrap(),
                        ),
                        ..Default::default()
                    };

                    let channels = lhs.next().unwrap();
                    for kv in channels.into_inner() {
                        let mut parts = kv.into_inner();
                        let k = parts.next().unwrap().as_str().parse::<usize>().unwrap();
                        let v = parts.next().unwrap().as_str().parse::<usize>().unwrap();
                        pixel.channels.push((k, v))
                    }

                    //println!("PIXEL '{}' -> '{}'", name, value);
                    kll.pixelmap.insert(index, pixel);
                }
                Rule::animdef => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let rhs = parts.next().unwrap();

                    let name = lhs.next().unwrap().as_str();

                    let mut animation = Animation::default();
                    for item in rhs.into_inner() {
                        animation.modifiers.push(item.as_str());
                    }

                    //println!("New animation '{}' -> '{}'", name, value);
                    kll.animations.insert(name, animation);
                }
                Rule::animframe => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let mut rhs = parts.next().unwrap().into_inner();

                    let name = lhs.next().unwrap().as_str();
                    let index = lhs.next().unwrap().as_str().parse::<usize>().unwrap();

                    let mut rparts = rhs.next().unwrap().into_inner();
                    let pos = rparts.next().unwrap().as_str();
                    let channels = rparts.next().unwrap();

                    let mut values = vec![];
                    for c in channels.into_inner() {
                        let color = c.as_str();
                        let value = match &color[0..1] {
                            "+" | "-" => PixelColor::Relative(color.parse::<isize>().unwrap()),
                            _ => PixelColor::Rgb(parse_int(color)),
                        };
                        values.push(value);
                    }
                    let pixel = Pixel {
                        range: PixelRange {
                            index: Some(PixelAddr::Absolute(parse_int(pos))),
                            ..Default::default()
                        },
                        channel_values: values,
                    };

                    //println!("New frame '{}' -> '{}'", name, value);
                    let animation = kll.animations.entry(name).or_default();
                    let frames = &mut animation.frames;
                    if frames.len() <= index {
                        frames.resize(index + 1, Pixel::default());
                    }
                    frames[index] = pixel;
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        Ok(kll)
    }
}

pub fn parse(text: &str) -> Result<KllState, PestError> {
    KllState::from_str(text)
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
