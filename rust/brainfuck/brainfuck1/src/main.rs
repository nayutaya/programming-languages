use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    MoveToRight,
    MoveToLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopBegin,
    LoopEnd,
    Invalid(char),
}

impl From<char> for Instruction {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Instruction::MoveToRight,
            '<' => Instruction::MoveToLeft,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
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

struct Tape {
    map: HashMap<usize, u8>,
    head: usize,
}

impl Tape {
    fn new() -> Self {
        Tape {
            map: HashMap::new(),
            head: 0,
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn increment(&mut self) {
        let entry = self.map.entry(self.head).or_insert(0);
        *entry += 1;
    }

    fn decrement(&mut self) {
        let entry = self.map.entry(self.head).or_insert(0);
        *entry -= 1;
    }
}

struct Context {
    tape: Tape,
}

impl Context {
    fn new() -> Self {
        Context {
            tape: Tape::new(),
        }
    }
}

fn execute(context: Context, insts: &Vec<Instruction>) -> Context {
    let mut context = context;
    for inst in insts {
        match inst {
            Instruction::Increment => { context.tape.increment(); }
            Instruction::Decrement => { context.tape.decrement(); }
            _ => {}
        }
    }
    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_char_into_inst() {
        assert_eq!(Instruction::MoveToRight,  '>'.into());
        assert_eq!(Instruction::MoveToLeft,   '<'.into());
        assert_eq!(Instruction::Increment,    '+'.into());
        assert_eq!(Instruction::Decrement,    '-'.into());
        assert_eq!(Instruction::LoopBegin,    '['.into());
        assert_eq!(Instruction::LoopEnd,      ']'.into());
        assert_eq!(Instruction::Output,       '.'.into());
        assert_eq!(Instruction::Input,        ','.into());
        assert_eq!(Instruction::Invalid(' '), ' '.into());
        assert_eq!(Instruction::Invalid('x'), 'x'.into());
    }

    #[test]
    fn can_parse_string() {
        // assert_eq!(Vec::new(), parse_instructions(""));
        assert!(parse_instructions("") == Vec::new());
        assert_eq!(
            vec![
                Instruction::MoveToRight,
                Instruction::MoveToLeft,
                Instruction::Increment,
                Instruction::Decrement,
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
        let insts = Vec::new();
        let context = execute(Context::new(), &insts);
        assert_eq!(true, context.tape.is_empty());
    }

    #[test]
    fn one() {
        let insts = vec![Instruction::Increment];
        let context = execute(Context::new(), &insts);
        let mut tape = HashMap::new();
        tape.insert(0, 1);
        assert_eq!(tape, context.tape.map);
    }

    #[test]
    fn zero() {
        let insts = vec![
            Instruction::Increment,
            Instruction::Decrement,
        ];
        let context = execute(Context::new(), &insts);
        let mut tape = HashMap::new();
        tape.insert(0, 0);
        assert_eq!(tape, context.tape.map);
    }

    /*
    #[test]
    fn print_space() {
        let context1 = Context::new();
        let insts = Vec::new();
        for i in 1..=32 { insts.push(Instruction::Increment); }
        let context2 = execute(context1, &insts);
        // assert_eq!(true, context2.tape.is_empty());
    }
    */
}

fn main() {
    println!("Hello, world!");
}
