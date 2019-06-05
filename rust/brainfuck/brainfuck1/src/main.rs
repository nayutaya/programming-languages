use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    Increment,
    Decrement,
    PointerIncrement,
    PointerDecrement,
    Output,
    Input,
    LoopBegin,
    LoopEnd,
    Invalid(char),
}

impl From<char> for Instruction {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Instruction::Increment,
            '<' => Instruction::Decrement,
            '+' => Instruction::PointerIncrement,
            '-' => Instruction::PointerDecrement,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::LoopBegin,
            ']' => Instruction::LoopEnd,
            _   => Instruction::Invalid(ch),
        }
    }
}

fn parse_instructions(inst: &str) -> Vec<Instruction> {
    inst.chars().map(|ch| ch.into()).collect()
}

struct Context {
    tape: HashMap<usize, u8>,
}

impl Context {
    fn new() -> Self {
        Context {
            tape: HashMap::new(),
        }
    }
}

fn execute(context: Context, insts: &Vec<Instruction>) -> Context {
    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_char_into_inst() {
        assert_eq!(Instruction::Increment,        '>'.into());
        assert_eq!(Instruction::Decrement,        '<'.into());
        assert_eq!(Instruction::PointerIncrement, '+'.into());
        assert_eq!(Instruction::PointerDecrement, '-'.into());
        assert_eq!(Instruction::LoopBegin,        '['.into());
        assert_eq!(Instruction::LoopEnd,          ']'.into());
        assert_eq!(Instruction::Output,           '.'.into());
        assert_eq!(Instruction::Input,            ','.into());
        assert_eq!(Instruction::Invalid(' '),     ' '.into());
        assert_eq!(Instruction::Invalid('x'),     'x'.into());
    }

    #[test]
    fn can_parse_string() {
        // assert_eq!(Vec::new(), parse_instructions(""));
        assert!(parse_instructions("") == Vec::new());
        assert_eq!(
            vec![
                Instruction::Increment,
                Instruction::Decrement,
                Instruction::PointerIncrement,
                Instruction::PointerDecrement,
                Instruction::Output,
                Instruction::Input,
                Instruction::LoopBegin,
                Instruction::LoopEnd,
                Instruction::Invalid(' '),
            ],
            parse_instructions("><+-.,[] "));
    }

    #[test]
    fn empty() {
        let context1 = Context::new();
        let context2 = execute(context1, &Vec::new());
        assert_eq!(true, context2.tape.is_empty());
    }
}

fn main() {
    println!("Hello, world!");
}
