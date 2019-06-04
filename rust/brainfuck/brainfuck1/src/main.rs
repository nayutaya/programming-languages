#[derive(Debug, PartialEq)]
enum Instruction {
    Increment,
    Invalid,
}

fn parse_char(ch: char) -> Instruction {
    match ch {
        '>' => Instruction::Increment,
        _   => Instruction::Invalid,
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
        assert_eq!(Instruction::Invalid, parse_char(' '));
    }

    #[test]
    fn can_parse_string() {
        // assert_eq!(Vec::new(), parse_instructions(""));
        assert!(parse_instructions("") == Vec::new());
        assert_eq!(
            vec![
                Instruction::Increment,
                Instruction::Invalid,
            ],
            parse_instructions("> "));
    }
}

fn main() {
    println!("Hello, world!");
}
