use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

type PestError = pest::error::Error<Rule>;

#[derive(Parser)]
#[grammar = "kll.pest"]
pub struct KLLParser;

#[derive(Debug, Default)]
pub struct KllState<'a> {
    variables: HashMap<&'a str, &'a str>,
    defines: HashMap<&'a str, &'a str>,
    capabilities: HashMap<&'a str, &'a str>,
    keymap: HashMap<&'a str, &'a str>,
    positions: HashMap<&'a str, &'a str>,
    pixelmap: HashMap<&'a str, &'a str>,
    animations: HashMap<&'a str, &'a str>,
    frames: HashMap<&'a str, Vec<&'a str>>,
}

impl KllState<'_> {
    fn from_str(text: &str) -> Result<KllState, PestError> {
        let file = KLLParser::parse(Rule::file, text)?;

        let mut kll = KllState::default();

        for line in file {
            match line.as_rule() {
                Rule::property => {
                    let mut parts = line.into_inner();
                    let name: &str = parts.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("SET '{}' to '{}'", name, value);
                    kll.variables.insert(name, value);
                }
                Rule::define => {
                    let mut parts = line.into_inner();
                    let name: &str = parts.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("DEFINE '{}' to '{}'", name, value);
                    kll.defines.insert(name, value);
                }
                Rule::capability => {
                    let mut parts = line.into_inner();
                    let name: &str = parts.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("CAP '{}' -> '{}'", name, value);
                    kll.capabilities.insert(name, value);
                }
                Rule::mapping => {
                    let mut parts = line.into_inner();
                    let trigger: &str = parts.next().unwrap().as_str();
                    let kind: &str = parts.next().unwrap().as_str();
                    let result: &str = parts.next().unwrap().as_str();
                    //println!("BIND '{}' '{}' '{}'", trigger, kind, result);
                    kll.keymap.insert(trigger, result);
                }
                Rule::position => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let name = lhs.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("POS '{}' -> '{}'", name, value);
                    kll.positions.insert(name, value);
                }
                Rule::pixelmap => {
                    let mut parts = line.into_inner();
                    let name: &str = parts.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("PIXEL '{}' -> '{}'", name, value);
                    kll.pixelmap.insert(name, value);
                }
                Rule::animdef => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let name = lhs.next().unwrap().as_str();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("New animation '{}' -> '{}'", name, value);
                    kll.animations.insert(name, value);
                }
                Rule::animframe => {
                    let mut parts = line.into_inner();
                    let mut lhs = parts.next().unwrap().into_inner();
                    let name = lhs.next().unwrap().as_str();
                    let index = lhs.next().unwrap().as_str().parse::<usize>().unwrap();
                    let value: &str = parts.next().unwrap().as_str();
                    //println!("New frame '{}' -> '{}'", name, value);
                    let frames = kll.frames.entry(name).or_default();
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
