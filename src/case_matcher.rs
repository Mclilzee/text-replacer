enum Case {
    Capitalized,
    Lowercase,
    Uppercase,
}

pub fn match_case(first: &str, second: &str) -> String {
    let case = find_case(first);
    match case {
        Case::Lowercase => second.to_ascii_lowercase(),
        Case::Uppercase => second.to_ascii_uppercase(),
        Case::Capitalized => [
            second[0..1].to_ascii_uppercase(),
            second[1..].to_ascii_lowercase(),
        ]
        .concat(),
    }
}

fn find_case(string: &str) -> Case {
    let mut chars = string.chars();
    if string.is_empty() || chars.next().unwrap().is_ascii_lowercase() {
        return Case::Lowercase;
    }

    match chars.find(|b| b.is_ascii_lowercase()) {
        Some(_) => Case::Capitalized,
        None => Case::Uppercase,
    }
}

#[cfg(test)]
mod test {
    use crate::case_matcher::match_case;

    #[test]
    fn return_lowercase() {
        assert_eq!("lowertoo", match_case("lower", "lowertoo"));
    }

    #[test]
    fn return_uppercase() {
        assert_eq!("TEXT", match_case("UPPER", "text"));
    }

    #[test]
    fn return_capitalized() {
        assert_eq!("Capitalized", match_case("This", "capitalized"));
    }

    #[test]
    fn return_lower_case_if_case_mixed() {
        assert_eq!("lowercase", match_case("tHiS", "lowercase"));
    }

    #[test]
    fn works_with_one_letter() {
        assert_eq!("lower", match_case("s", "lower"));
        assert_eq!("UPPER", match_case("S", "upper"));
    }

    #[test]
    fn convert_text_to_correct_case() {
        assert_eq!("lower", match_case("lower", "LOWER"));
        assert_eq!("UPPER", match_case("UPPER", "upper"));
        assert_eq!("Cap", match_case("Capitalized", "cAP"));
    }
}
