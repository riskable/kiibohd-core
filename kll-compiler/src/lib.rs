pub mod emitters;
pub mod parser;
mod test;
pub mod types;

use parser::PestError;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;
use types::{
    Action, Animation, AnimationResult, Capability, Key, KllFile, PixelDef, Position, ResultType,
    Statement, Trigger, TriggerMode, TriggerType,
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
    pub keymap: Vec<(Vec<Vec<Trigger<'a>>>, TriggerMode, Vec<Vec<Action<'a>>>)>,
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
                Statement::Keymap((triggers, varient, actions)) => {
                    kll.keymap.push((triggers, varient, actions));
                }
                Statement::Position((indices, pos)) => {
                    for range in indices {
                        for index in range {
                            kll.positions.insert(index, pos.clone());
                        }
                    }
                }
                Statement::Pixelmap((indices, map)) => {
                    for range in indices {
                        for index in range {
                            kll.pixelmap.insert(index, map.clone());
                        }
                    }
                }
                Statement::Animation((name, anim)) => {
                    kll.animations.insert(name, anim);
                }
                Statement::Frame((name, indices, frame)) => {
                    let animation = kll.animations.entry(name).or_default();
                    let frames = &mut animation.frames;
                    for range in indices {
                        for index in range {
                            if frames.len() <= index {
                                frames.resize(index + 1, vec![]);
                            }
                            frames[index] = frame.clone();
                        }
                    }
                }
                Statement::NOP => {}
            };
        }

        kll
    }
}

impl<'a> KllState<'a> {
    pub fn triggers(&self) -> impl Iterator<Item = &Trigger> + '_ {
        let groups = self
            .keymap
            .iter()
            .map(|(trigger_groups, _, _)| trigger_groups);
        let combos: Vec<_> = groups.into_iter().flatten().collect();
        let triggers = combos.into_iter().flatten();
        triggers
    }

    pub fn actions(&self) -> impl Iterator<Item = &Action> + '_ {
        let groups = self
            .keymap
            .iter()
            .map(|(_, _, result_groups)| result_groups);
        let combos: Vec<_> = groups.into_iter().flatten().collect();
        let actions = combos.into_iter().flatten();
        actions
    }

    pub fn scancodes(&self) -> Vec<usize> {
        self.triggers()
            .filter_map(|t| match &t.trigger {
                TriggerType::Key(key) => Some(key),
                _ => None,
            })
            .filter_map(|key| match key {
                Key::Scancode(s) => Some(*s),
                _ => None,
            })
            .collect()
    }

    pub fn animations(&self) -> impl Iterator<Item = &AnimationResult> + '_ {
        self.actions().filter_map(|action| match &action.result {
            ResultType::Animation(anim) => Some(anim),
            _ => None,
        })
    }

    pub fn unicode_strings(&self) -> HashSet<String> {
        self.actions()
            .filter_map(|action| match &action.result {
                ResultType::UnicodeText(text) => Some(text.to_string()),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct KllDatastore<'a> {
    pub scancode_range: Range<usize>,
    pub unicode_strings: HashSet<String>,
    pub unique_triggers: HashSet<Trigger<'a>>,
    pub unique_results: HashSet<Action<'a>>,
    pub unique_animations: HashSet<AnimationResult<'a>>,
}

impl<'a> KllDatastore<'a> {
    pub fn get_scancode_range(state: &KllState) -> Range<usize> {
        let mut range = Range {
            start: 0xFFFF,
            end: 0,
        };

        for scancode in state.scancodes() {
            //dbg!(scancode);
            if scancode < range.start {
                range.start = scancode;
            }
            if scancode > range.end {
                range.end = scancode;
            }
        }

        assert!(range.start < range.end);
        range
    }

    pub fn new(state: &'a KllState<'a>) -> KllDatastore<'a> {
        KllDatastore {
            unicode_strings: state.unicode_strings(),
            scancode_range: KllDatastore::get_scancode_range(&state),
            unique_triggers: state.triggers().map(|x| x.clone()).collect(),
            unique_results: state.actions().map(|x| x.clone()).collect(),
            unique_animations: state.animations().map(|x| x.clone()).collect(),
        }
    }
}

pub fn parse(text: &str) -> Result<KllFile, PestError> {
    KllFile::from_str(text)
}
