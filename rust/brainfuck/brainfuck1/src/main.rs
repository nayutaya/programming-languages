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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_char_into_inst() {
        assert_eq!(Instruction::Increment, '>'.into());
        assert_eq!(Instruction::Decrement, '<'.into());
        assert_eq!(Instruction::PointerIncrement, '+'.into());
        assert_eq!(Instruction::PointerDecrement, '-'.into());
        assert_eq!(Instruction::LoopBegin, '['.into());
        assert_eq!(Instruction::LoopEnd, ']'.into());
        assert_eq!(Instruction::Output, '.'.into());
        assert_eq!(Instruction::Input, ','.into());
        assert_eq!(Instruction::Invalid(' '), ' '.into());
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
