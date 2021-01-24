use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

type PestError = pest::error::Error<Rule>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

#[derive(Debug)]
pub enum Variable<'a> {
    List(Vec<&'a str>),
    String(&'a str),
}

#[derive(Debug, Default)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
    rx: usize,
    ry: usize,
    rz: usize,
}

#[derive(Debug, Default)]
pub struct PixelDef {
    channels: Vec<(usize, usize)>,
    scancode: Option<String>,
}

#[derive(Debug, Default)]
pub struct Animation<'a> {
    modifiers: Vec<&'a str>,
    frames: Vec<&'a str>,
}

#[derive(Debug, Default)]
pub struct Capability<'a> {
    function: &'a str,
    args: Vec<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Trigger<'a> {
    Key(usize),
    Other(&'a str),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action<'a> {
    Usb(&'a str),
    Consumer(&'a str),
    System(&'a str),
    Other(&'a str),
}

#[derive(Debug, Default)]
pub struct KllState<'a> {
    defines: HashMap<&'a str, &'a str>,
    variables: HashMap<&'a str, Variable<'a>>,
    capabilities: HashMap<&'a str, Capability<'a>>,
    keymap: HashMap<Trigger<'a>, Action<'a>>,
    positions: HashMap<usize, Position>,
    pixelmap: HashMap<&'a str, PixelDef>,
    animations: HashMap<&'a str, Animation<'a>>,
}

fn parse_int(s: &str) -> usize {
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
                    let mode = parts.next().unwrap().as_str();
                    let rhs = parts.next().unwrap();

                    let text = lhs.as_str();
                    let trigger = match lhs.into_inner().next().unwrap().as_rule() {
                        Rule::key_trigger => {
                            let key = text.strip_prefix("S").unwrap();
                            Trigger::Key(parse_int(key))
                        }
                        _ => unimplemented!(),
                    };

                    let text = rhs.as_str();
                    let result = match rhs.into_inner().next().unwrap().as_rule() {
                        Rule::usbcode => Action::Usb(text.strip_prefix("U").unwrap()),
                        Rule::consumer => Action::Consumer(text.strip_prefix("CON").unwrap()),
                        Rule::system => Action::System(text.strip_prefix("SYS").unwrap()),
                        Rule::color => Action::Other(text),
                        _ => unimplemented!(),
                    };

                    kll.keymap.insert(trigger, result);
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

                    let name = lhs.next().unwrap().as_str();
                    let mut pixel = PixelDef {
                        scancode: Some(scancode.to_string()),
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
                    kll.pixelmap.insert(name, pixel);
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
                    let name = lhs.next().unwrap().as_str();
                    let index = lhs.next().unwrap().as_str().parse::<usize>().unwrap();
                    let value = parts.next().unwrap().as_str();
                    //println!("New frame '{}' -> '{}'", name, value);
                    let animation = kll.animations.entry(name).or_default();
                    let frames = &mut animation.frames;
                    if frames.len() <= index {
                        frames.resize(index + 1, "");
                    }
                    frames[index] = value;
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
