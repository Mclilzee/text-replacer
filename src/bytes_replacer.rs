use crate::case_matcher::match_case;
use crate::utils::{get_be_16_bytes, get_le_16_bytes};
use std::collections::HashMap;

pub fn replace_ascii(dictionary: &HashMap<String, String>, bytes: &[u8]) -> (u128, Vec<u8>) {
    let mut new_bytes: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    let mut count = 0;
    while cursor < bytes.len() {
        let char = char::from(bytes[cursor]);
        if char.is_ascii_alphanumeric() {
            let word: String = bytes[cursor..]
                .iter()
                .map(|&b| char::from(b))
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();

            cursor += word.len();
            let new_word = match dictionary.get(&word.to_lowercase()) {
                Some(value) => {
                    count += 1;
                    &match_case(&word, value)
                }
                None => &word,
            };

            new_bytes.extend_from_slice(new_word.as_bytes());
        } else {
            new_bytes.push(bytes[cursor]);
            cursor += 1;
        }
    }

    (count, new_bytes)
}

pub fn replace_le_16(dictionary: &HashMap<String, String>, bytes: &[u8]) -> (u128, Vec<u8>) {
    let mut new_bytes: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    let mut count = 0;
    while cursor < bytes.len() - 1 {
        let u16_bytes = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        let char = char::from_u32(u16_bytes as u32).unwrap_or(';');
        if char.is_ascii_alphanumeric() {
            let mut chars: Vec<char> = vec![];
            while cursor < bytes.len() - 1 {
                let u16_bytes = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
                let char = char::from_u32(u16_bytes as u32).unwrap_or(';');
                if char.is_ascii_alphanumeric() || char == '_' {
                    cursor += 2;
                    chars.push(char);
                } else {
                    break;
                }
            }

            let word = chars.iter().collect::<String>();
            let new_word = match dictionary.get(&word.to_lowercase()) {
                Some(value) => {
                    count += 1;
                    &match_case(&word, value)
                }
                None => &word,
            };

            let word_u16_bytes: Vec<u8> = get_le_16_bytes(new_word);
            word_u16_bytes.into_iter().for_each(|b| new_bytes.push(b));
        } else {
            new_bytes.push(bytes[cursor]);
            cursor += 1;

            if cursor == bytes.len() - 1 {
                new_bytes.push(bytes[cursor]);
            }
        }
    }

    (count, new_bytes)
}

pub fn replace_be_16(dictionary: &HashMap<String, String>, bytes: &[u8]) -> (u128, Vec<u8>) {
    let mut new_bytes: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    let mut count = 0;
    while cursor < bytes.len() - 1 {
        let u16_bytes = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
        let char = char::from_u32(u16_bytes as u32).unwrap_or(';');
        if char.is_ascii_alphanumeric() {
            let mut chars: Vec<char> = vec![];
            while cursor < bytes.len() - 1 {
                let u16_bytes = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]);
                let char = char::from_u32(u16_bytes as u32).unwrap_or(';');
                if char.is_ascii_alphanumeric() || char == '_' {
                    cursor += 2;
                    chars.push(char);
                } else {
                    break;
                }
            }

            let word = chars.iter().collect::<String>();
            let new_word = match dictionary.get(&word.to_lowercase()) {
                Some(value) => {
                    count += 1;
                    &match_case(&word, value)
                }
                None => &word,
            };

            let word_u16_bytes: Vec<u8> = get_be_16_bytes(new_word);
            word_u16_bytes.into_iter().for_each(|b| new_bytes.push(b));
        } else {
            new_bytes.push(bytes[cursor]);
            cursor += 1;

            if cursor == bytes.len() - 1 {
                new_bytes.push(bytes[cursor]);
            }
        }
    }

    (count, new_bytes)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::bytes_replacer::{replace_ascii, replace_be_16, replace_le_16};
    use crate::utils::{get_be_16_bytes, get_le_16_bytes};

    #[test]
    fn replaces_text_le() {
        let map = create_map();
        let content = get_le_16_bytes("lorem pondem first");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(get_le_16_bytes("lorem pondem changed"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn works_with_stoppers_le() {
        let map = create_map();
        let content = get_le_16_bytes("lorem pondem first.");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(get_le_16_bytes("lorem pondem changed."), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn change_more_words_le() {
        let map = create_map();
        let content = get_le_16_bytes("first and another.");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(get_le_16_bytes("changed and something."), result);
        assert_eq!(count, 2);
    }

    #[test]
    fn correctly_works_with_plural_le() {
        let map = create_map();
        let content = get_le_16_bytes("first and another, anothers.");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(
            get_le_16_bytes("changed and something, somethingelse."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn new_lines_work_le() {
        let map = create_map();
        let content = get_le_16_bytes("\nfirst and \nAnother, \nANOTHERS.");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(
            get_le_16_bytes("\nchanged and \nSomething, \nSOMETHINGELSE."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn capilization_le() {
        let map = create_map();
        let content = get_le_16_bytes("First Another Capital Anothers");
        let (_, result) = replace_le_16(&map, &content);
        assert_eq!(
            get_le_16_bytes("Changed Something Toonice Somethingelse"),
            result
        );
    }

    #[test]
    fn uppercase_le() {
        let map = create_map();
        let content = get_le_16_bytes("FIRST ANOTHER CAPITAL ANOTHERS");
        let (_, result) = replace_le_16(&map, &content);
        assert_eq!(
            get_le_16_bytes("CHANGED SOMETHING TOONICE SOMETHINGELSE"),
            result
        );
    }

    #[test]
    fn lowercase_le() {
        let map = create_map();
        let content = get_le_16_bytes("first another capital anothers");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(
            get_le_16_bytes("changed something toonice somethingelse"),
            result
        );
        assert_eq!(count, 4);
    }

    #[test]
    fn replacment_is_bigger_than_original_le() {
        let map = create_map();
        let content = get_le_16_bytes("small");
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(get_le_16_bytes("this is bigger than the original"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn replace_asian_and_russian_chars_le() {
        let map = create_map();
        let content = get_le_16_bytes(
            "1 string m_Localized = \"Chinese (中文)\"\n0 MetadataCollection m_Metadata\n1 string m_Localized = \"Russian (Русский)\"\n0 MetadataCollection m_Metadata",
        );
        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(content, result);
        assert_eq!(count, 0);
    }

    #[test]
    fn add_bytes_in_beggining_le() {
        let map = create_map();
        let content = get_le_16_bytes("small");
        let expected = get_le_16_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [prefix.clone(), content].concat();
        let expected = [prefix, expected].concat();

        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_to_the_end_le() {
        let map = create_map();
        let content = get_le_16_bytes("small");
        let expected = get_le_16_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [content, prefix.clone()].concat();
        let expected = [expected, prefix].concat();

        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_in_the_mid_le() {
        let map = create_map();
        let bytes1 = vec![11, 9];
        let bytes2 = vec![2];
        let bytes3 = vec![255, 249];
        let bytes4 = vec![10];

        let content1 = get_le_16_bytes("\nfirst");
        let content2 = get_le_16_bytes(" and \nAnother,");
        let content3 = get_le_16_bytes(" \nANOTHERS.");
        let content = [
            bytes1.clone(),
            content1,
            bytes2.clone(),
            content2,
            bytes3.clone(),
            content3,
            bytes4.clone(),
        ]
        .concat();

        let expected1 = get_le_16_bytes("\nchanged");
        let expected2 = get_le_16_bytes(" and \nSomething,");
        let expected3 = get_le_16_bytes(" \nSOMETHINGELSE.");
        let expected = [
            bytes1, expected1, bytes2, expected2, bytes3, expected3, bytes4,
        ]
        .concat();

        let (count, result) = replace_le_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 3);
    }

    #[test]
    fn replaces_text_be() {
        let map = create_map();
        let content = get_be_16_bytes("lorem pondem first");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(get_be_16_bytes("lorem pondem changed"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn works_with_stoppers_be() {
        let map = create_map();
        let content = get_be_16_bytes("lorem pondem first.");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(get_be_16_bytes("lorem pondem changed."), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn change_more_words_be() {
        let map = create_map();
        let content = get_be_16_bytes("first and another.");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(get_be_16_bytes("changed and something."), result);
        assert_eq!(count, 2);
    }

    #[test]
    fn correctly_works_with_plural_be() {
        let map = create_map();
        let content = get_be_16_bytes("first and another, anothers.");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(
            get_be_16_bytes("changed and something, somethingelse."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn new_lines_work_be() {
        let map = create_map();
        let content = get_be_16_bytes("\nfirst and \nAnother, \nANOTHERS.");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(
            get_be_16_bytes("\nchanged and \nSomething, \nSOMETHINGELSE."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn capilization_be() {
        let map = create_map();
        let content = get_be_16_bytes("First Another Capital Anothers");
        let (_, result) = replace_be_16(&map, &content);
        assert_eq!(
            get_be_16_bytes("Changed Something Toonice Somethingelse"),
            result
        );
    }

    #[test]
    fn uppercase_be() {
        let map = create_map();
        let content = get_be_16_bytes("FIRST ANOTHER CAPITAL ANOTHERS");
        let (_, result) = replace_be_16(&map, &content);
        assert_eq!(
            get_be_16_bytes("CHANGED SOMETHING TOONICE SOMETHINGELSE"),
            result
        );
    }

    #[test]
    fn lowercase_be() {
        let map = create_map();
        let content = get_be_16_bytes("first another capital anothers");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(
            get_be_16_bytes("changed something toonice somethingelse"),
            result
        );
        assert_eq!(count, 4);
    }

    #[test]
    fn replacment_is_bigger_than_original_be() {
        let map = create_map();
        let content = get_be_16_bytes("small");
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(get_be_16_bytes("this is bigger than the original"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn replace_asian_and_russian_chars_be() {
        let map = create_map();
        let content = get_be_16_bytes(
            "1 string m_Localized = \"Chinese (中文)\"\n0 MetadataCollection m_Metadata\n1 string m_Localized = \"Russian (Русский)\"\n0 MetadataCollection m_Metadata",
        );
        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(content, result);
        assert_eq!(count, 0);
    }

    #[test]
    fn add_bytes_in_beggining_be() {
        let map = create_map();
        let content = get_be_16_bytes("small");
        let expected = get_be_16_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [prefix.clone(), content].concat();
        let expected = [prefix, expected].concat();

        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_to_the_end_be() {
        let map = create_map();
        let content = get_be_16_bytes("small");
        let expected = get_be_16_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [content, prefix.clone()].concat();
        let expected = [expected, prefix].concat();

        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_in_the_mid_be() {
        let map = create_map();
        let bytes1 = vec![11, 9];
        let bytes2 = vec![2];
        let bytes3 = vec![255, 249];
        let bytes4 = vec![10];

        let content1 = get_be_16_bytes("\nfirst");
        let content2 = get_be_16_bytes(" and \nAnother,");
        let content3 = get_be_16_bytes(" \nANOTHERS.");
        let content = [
            bytes1.clone(),
            content1,
            bytes2.clone(),
            content2,
            bytes3.clone(),
            content3,
            bytes4.clone(),
        ]
        .concat();

        let expected1 = get_be_16_bytes("\nchanged");
        let expected2 = get_be_16_bytes(" and \nSomething,");
        let expected3 = get_be_16_bytes(" \nSOMETHINGELSE.");
        let expected = [
            bytes1, expected1, bytes2, expected2, bytes3, expected3, bytes4,
        ]
        .concat();

        let (count, result) = replace_be_16(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 3);
    }

    #[test]
    fn replaces_text_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("lorem pondem first");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(get_ascii_bytes("lorem pondem changed"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn works_with_stoppers_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("lorem pondem first.");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(get_ascii_bytes("lorem pondem changed."), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn change_more_words_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("first and another.");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(get_ascii_bytes("changed and something."), result);
        assert_eq!(count, 2);
    }

    #[test]
    fn correctly_works_with_plural_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("first and another, anothers.");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(
            get_ascii_bytes("changed and something, somethingelse."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn new_lines_work_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("\nfirst and \nAnother, \nANOTHERS.");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(
            get_ascii_bytes("\nchanged and \nSomething, \nSOMETHINGELSE."),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn capilization_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("First Another Capital Anothers");
        let (_, result) = replace_ascii(&map, &content);
        assert_eq!(
            get_ascii_bytes("Changed Something Toonice Somethingelse"),
            result
        );
    }

    #[test]
    fn uppercase_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("FIRST ANOTHER CAPITAL ANOTHERS");
        let (_, result) = replace_ascii(&map, &content);
        assert_eq!(
            get_ascii_bytes("CHANGED SOMETHING TOONICE SOMETHINGELSE"),
            result
        );
    }

    #[test]
    fn lowercase_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("first another capital anothers");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(
            get_ascii_bytes("changed something toonice somethingelse"),
            result
        );
        assert_eq!(count, 4);
    }

    #[test]
    fn replacment_is_bigger_than_original_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("small");
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(get_ascii_bytes("this is bigger than the original"), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn replace_asian_and_russian_chars_ascii() {
        let map = create_map();
        let content = get_ascii_bytes(
            "1 string m_Localized = \"Chinese (中文)\"\n0 MetadataCollection m_Metadata\n1 string m_Localized = \"Russian (Русский)\"\n0 MetadataCollection m_Metadata",
        );
        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(content, result);
        assert_eq!(count, 0);
    }

    #[test]
    fn add_bytes_in_beggining_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("small");
        let expected = get_ascii_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [prefix.clone(), content].concat();
        let expected = [prefix, expected].concat();

        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_to_the_end_ascii() {
        let map = create_map();
        let content = get_ascii_bytes("small");
        let expected = get_ascii_bytes("this is bigger than the original");
        let prefix: Vec<u8> = vec![22, 1, 255];
        let content = [content, prefix.clone()].concat();
        let expected = [expected, prefix].concat();

        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 1);
    }

    #[test]
    fn add_bytes_in_the_mid_ascii() {
        let map = create_map();
        let bytes1 = vec![11, 9];
        let bytes2 = vec![2];
        let bytes3 = vec![255, 249];
        let bytes4 = vec![10];

        let content1 = get_ascii_bytes("\nfirst");
        let content2 = get_ascii_bytes(" and \nAnother,");
        let content3 = get_ascii_bytes(" \nANOTHERS.");
        let content = [
            bytes1.clone(),
            content1,
            bytes2.clone(),
            content2,
            bytes3.clone(),
            content3,
            bytes4.clone(),
        ]
        .concat();

        let expected1 = get_ascii_bytes("\nchanged");
        let expected2 = get_ascii_bytes(" and \nSomething,");
        let expected3 = get_ascii_bytes(" \nSOMETHINGELSE.");
        let expected = [
            bytes1, expected1, bytes2, expected2, bytes3, expected3, bytes4,
        ]
        .concat();

        let (count, result) = replace_ascii(&map, &content);
        assert_eq!(expected, result);
        assert_eq!(count, 3);
    }

    fn create_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("first".into(), "changed".into());
        map.insert("another".into(), "something".into());
        map.insert("capital".into(), "toonice".into());
        map.insert("anothers".into(), "somethingelse".into());
        map.insert("small".into(), "this is bigger than the original".into());
        map
    }

    fn get_ascii_bytes(text: &str) -> Vec<u8> {
        text.as_bytes().to_vec()
    }
}
