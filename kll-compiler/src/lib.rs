pub mod emitters;
pub mod parser;
mod test;
pub mod types;

use parser::PestError;
use std::collections::HashMap;
use types::{
    Action, Animation, Capability, KllFile, PixelDef, Position, Statement, Trigger, TriggerVarient,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value<'a> {
    List(Vec<&'a str>),
    Single(&'a str),
}

#[derive(Debug, Default, Clone)]
pub struct KllState<'a> {
    pub defines: HashMap<&'a str, &'a str>,
    pub variables: HashMap<&'a str, Value<'a>>,
    pub capabilities: HashMap<&'a str, Capability<'a>>,
    pub keymap: Vec<(Trigger<'a>, TriggerVarient, Action<'a>)>,
    pub positions: HashMap<usize, Position>,
    pub pixelmap: HashMap<usize, PixelDef>,
    pub animations: HashMap<&'a str, Animation<'a>>,
}

impl<'a> KllFile<'a> {
    pub fn into_struct(self) -> KllState<'a> {
        let mut kll = KllState::default();
        for statement in self.statements {
            match statement {
                Statement::Define((name, val)) => {
                    kll.defines.insert(name, val);
                }
                Statement::Variable((name, index, val)) => {
                    let entry = kll.variables.entry(name).or_insert_with(|| match index {
                        Some(_) => Value::List(vec![]),
                        None => Value::Single(val),
                    });
                    match entry {
                        Value::List(vec) => {
                            let index = index.unwrap(); // Should be set because this is an array
                            if index >= vec.len() {
                                vec.resize(index + 1, "");
                            }
                            vec[index] = val;
                        }
                        Value::Single(s) => {
                            *s = val;
                        }
                    };
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

pub fn parse(text: &str) -> Result<KllFile, PestError> {
    KllFile::from_str(text)
}
