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
    Ignore(char),
}

impl Instruction {
    fn parse(inst: &str) -> Vec<Instruction> {
        inst.chars().map(Instruction::from).collect()
    }
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
            _   => Instruction::Ignore(ch),
        }
    }
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

    /*
    fn len(&self) -> usize {
        self.map.len()
    }
    */

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn move_to_right(&mut self) {
        self.head += 1;
    }

    fn move_to_left(&mut self) {
        self.head -= 1;
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
    output: Vec<u8>,
}

impl Context {
    fn new() -> Self {
        Context {
            tape: Tape::new(),
            output: Vec::new(),
        }
    }

    fn output(&mut self) {
        if let Some(ch) = self.tape.map.get(&self.tape.head) {
            self.output.push(*ch);
        } else {
            self.output.push(0);
        }
    }
}

fn execute_with(context: Context, insts: &Vec<Instruction>) -> Context {
    let mut context = context;
    for inst in insts {
        match inst {
            Instruction::MoveToRight => { context.tape.move_to_right(); }
            Instruction::MoveToLeft  => { context.tape.move_to_left(); }
            Instruction::Increment   => { context.tape.increment(); }
            Instruction::Decrement   => { context.tape.decrement(); }
            Instruction::Output      => { context.output(); }
            _ => {}
        }
    }
    context
}

fn execute(insts: &str) -> Context {
    execute_with(Context::new(), &Instruction::parse(&insts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_char_into_inst() {
        assert_eq!(Instruction::MoveToRight, '>'.into());
        assert_eq!(Instruction::MoveToLeft,  '<'.into());
        assert_eq!(Instruction::Increment,   '+'.into());
        assert_eq!(Instruction::Decrement,   '-'.into());
        assert_eq!(Instruction::LoopBegin,   '['.into());
        assert_eq!(Instruction::LoopEnd,     ']'.into());
        assert_eq!(Instruction::Output,      '.'.into());
        assert_eq!(Instruction::Input,       ','.into());
        assert_eq!(Instruction::Ignore(' '), ' '.into());
        assert_eq!(Instruction::Ignore('x'), 'x'.into());
    }

    #[test]
    fn can_parse_string() {
        // assert_eq!(Vec::new(), parse_instructions(""));
        assert!(Instruction::parse("") == Vec::new());
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
                Instruction::Ignore(' '),
            ],
            Instruction::parse("><+-.,[] "));
    }

    #[test]
    fn empty() {
        let context = execute("");
        assert_eq!(true, context.tape.is_empty());
        assert_eq!(0, context.tape.head);
        assert_eq!(0, context.output.len())
    }

    #[test]
    fn one() {
        let context = execute("+-+");
        let mut tape = HashMap::new();
        tape.insert(0, 1);
        assert_eq!(tape, context.tape.map);
        assert_eq!(0, context.tape.head);
        assert_eq!(0, context.output.len())
    }

    #[test]
    fn head1() {
        let context = execute("><>");
        assert_eq!(true, context.tape.is_empty());
        assert_eq!(1, context.tape.head);
        assert_eq!(0, context.output.len())
    }

    #[test]
    fn print_0() {
        let context = execute(".");
        assert_eq!(true, context.tape.is_empty());
        assert_eq!(0, context.tape.head);
        assert_eq!(vec![0u8], context.output);
    }

    #[test]
    fn print_1() {
        let context = execute("+.");
        assert_eq!(false, context.tape.is_empty());
        assert_eq!(0, context.tape.head);
        assert_eq!(vec![1u8], context.output);
    }

    /*
    #[test]
    fn print_a() {
        let context = execute("++++++[>++++++++++<-]>+++++.");
        assert_eq!(false, context.tape.is_empty());
        assert_eq!(1, context.tape.head);
        assert_eq!(vec![65u8], context.output);
    }
    */

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
