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
    if insts.len() == 0 { return context; }

    let mut pc: usize = 0;
    loop {
        match insts[pc] {
            Instruction::MoveToRight => { context.tape.move_to_right(); }
            Instruction::MoveToLeft  => { context.tape.move_to_left(); }
            Instruction::Increment   => { context.tape.increment(); }
            Instruction::Decrement   => { context.tape.decrement(); }
            Instruction::Output      => { context.output(); }
            _ => {}
        }
        pc += 1;
        if pc >= insts.len() { break; }
    }
    context
}

fn execute(insts: &str) -> Context {
    execute_with(Context::new(), &Instruction::parse(&insts))
}

#[derive(Debug, PartialEq)]
struct LoopError {}

fn make_jump_table(insts: &Vec<Instruction>) -> Result<HashMap<usize, usize>, LoopError> {
    let mut map = HashMap::new();
    let mut stack = Vec::new();
    for (index, inst) in insts.iter().enumerate() {
        match inst {
            Instruction::LoopBegin => {
                stack.push(index);
            }
            Instruction::LoopEnd => {
                if let Some(to) = stack.pop() {
                    map.insert(to, index);
                    map.insert(index, to);
                } else {
                    return Err(LoopError {});
                }
            }
            _ => { /* nop */ }
        }
    }
    if !stack.is_empty() {
        return Err(LoopError {});
    }
    Ok(map)
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

    #[test]
    fn empty_loop() {
        let context = execute("[]");
        assert_eq!(true, context.tape.is_empty());
        assert_eq!(0, context.tape.head);
        assert_eq!(0, context.output.len())
    }

    /*
    #[test]
    fn empty_loop3() {
        let context = execute("+++[.-]");
        assert_eq!(false, context.tape.is_empty());
        assert_eq!(0, context.tape.head);
        assert_eq!(vec![3u8, 2u8, 1u8], context.output);
    }
    */

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

    #[test]
    fn jump() {
        assert!(make_jump_table(&Instruction::parse("")) == Ok(HashMap::new()));
        {
            let mut map = HashMap::new();
            map.insert(0, 1);
            map.insert(1, 0);
            assert_eq!(Ok(map), make_jump_table(&Instruction::parse("[]")));
        }
        {
            let mut map = HashMap::new();
            map.insert(0, 3);
            map.insert(3, 0);
            map.insert(1, 2);
            map.insert(2, 1);
            assert_eq!(Ok(map), make_jump_table(&Instruction::parse("[[]]")));
        }
        assert_eq!(Err(LoopError {}), make_jump_table(&Instruction::parse("[")));
        assert_eq!(Err(LoopError {}), make_jump_table(&Instruction::parse("]")));
    }
}

fn main() {
    println!("Hello, world!");
}
