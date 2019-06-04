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

fn parse_char(ch: char) -> Instruction {
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

fn parse_instructions(inst: &str) -> Vec<Instruction> {
    inst.chars().map(parse_char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_char() {
        assert_eq!(Instruction::Increment, parse_char('>'));
        assert_eq!(Instruction::Decrement, parse_char('<'));
        assert_eq!(Instruction::PointerIncrement, parse_char('+'));
        assert_eq!(Instruction::PointerDecrement, parse_char('-'));
        assert_eq!(Instruction::LoopBegin, parse_char('['));
        assert_eq!(Instruction::LoopEnd, parse_char(']'));
        assert_eq!(Instruction::Output, parse_char('.'));
        assert_eq!(Instruction::Input, parse_char(','));
        assert_eq!(Instruction::Invalid(' '), parse_char(' '));
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
}

fn main() {
    println!("Hello, world!");
}
