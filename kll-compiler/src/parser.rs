use pest_consume::{match_nodes, Error, Parser};
use std::collections::HashMap;

use crate::types::*;

pub type PestError = Error<Rule>;
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

#[pest_consume::parser]
impl KLLParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
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

    pub fn file(input: Node) -> Result<Vec<Statement>> {
        Ok(match_nodes!(input.into_children();
            [statement(statements).., _] => statements.collect(),
        ))
    }
}

impl<'a> KllFile<'a> {
    pub fn from_str(text: &str) -> Result<KllFile> {
        let inputs = KLLParser::parse(Rule::file, text)?;
        let input = inputs.single()?;

        let kll = KllFile {
            statements: KLLParser::file(input)?,
        };

        Ok(kll)
    }
}
