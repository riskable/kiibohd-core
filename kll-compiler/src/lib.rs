mod parser;
mod types;
mod test;

use parser::PestError;
use std::collections::HashMap;
use types::{
    Action, Animation, Capability, KllFile, PixelDef, Position, Statement, Trigger, TriggerVarient,
    Variable,
};

#[derive(Debug, Default, Clone)]
pub struct KllState<'a> {
    pub defines: HashMap<&'a str, &'a str>,
    pub variables: HashMap<Variable<'a>, &'a str>,
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

pub fn parse(text: &str) -> Result<KllFile, PestError> {
    KllFile::from_str(text)
}
