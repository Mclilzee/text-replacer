use crate::case_matcher::match_case;
use std::collections::HashMap;
use std::iter;

pub fn text_replace(dictionary: &HashMap<String, String>, content: String) -> (u128, String) {
    let mut new_content = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut count = 0;
    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() {
            let word: String = iter::once(ch)
                .chain(iter::from_fn(|| {
                    chars.by_ref().next_if(|b| b.is_alphanumeric() || *b == '_')
                }))
                .collect();

            let word = match dictionary.get(&word.to_lowercase()) {
                Some(value) => {
                    count += 1;
                    &match_case(&word, value)
                }
                None => &word,
            };

            new_content += word;
        } else {
            new_content.push(ch);
        }
    }

    (count, new_content)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utf8_replacer::text_replace;

    #[test]
    fn replaces_text() {
        let map = create_map();
        let content = "lorem pondem first".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!("lorem pondem changed".to_owned(), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn works_with_stoppers() {
        let map = create_map();
        let content = "lorem pondem first.".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!("lorem pondem changed.".to_string(), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn change_more_words() {
        let map = create_map();
        let content = "first and another.".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!("changed and something.".to_string(), result);
        assert_eq!(count, 2);
    }

    #[test]
    fn correctly_works_with_plural() {
        let map = create_map();
        let content = "first and another, anothers.".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!("changed and something, somethingelse.".to_string(), result);
        assert_eq!(count, 3);
    }

    #[test]
    fn new_lines_work() {
        let map = create_map();
        let content = "\nfirst and \nAnother, \nANOTHERS.".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!(
            "\nchanged and \nSomething, \nSOMETHINGELSE.".to_string(),
            result
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn capilization() {
        let map = create_map();
        let content = "First Another Capital Anothers".to_string();
        let (_, result) = text_replace(&map, content);
        assert_eq!(
            "Changed Something Toonice Somethingelse".to_string(),
            result
        );
    }

    #[test]
    fn uppercase() {
        let map = create_map();
        let content = "FIRST ANOTHER CAPITAL ANOTHERS".to_string();
        let (_, result) = text_replace(&map, content);
        assert_eq!(
            "CHANGED SOMETHING TOONICE SOMETHINGELSE".to_string(),
            result
        );
    }

    #[test]
    fn lowercase() {
        let map = create_map();
        let content = "first another capital anothers".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!(
            "changed something toonice somethingelse".to_string(),
            result
        );
        assert_eq!(count, 4);
    }

    #[test]
    fn replacment_is_bigger_than_original() {
        let map = create_map();
        let content = "small".to_string();
        let (count, result) = text_replace(&map, content);
        assert_eq!("this is bigger than the original".to_string(), result);
        assert_eq!(count, 1);
    }

    #[test]
    fn replace_asian_and_russian_chars() {
        let map = create_map();
        let content = "1 string m_Localized = \"Chinese (中文)\"\n0 MetadataCollection m_Metadata\n1 string m_Localized = \"Russian (Русский)\"\n0 MetadataCollection m_Metadata".to_string();
        let (count, result) = text_replace(&map, content.clone());
        assert_eq!(content, result);
        assert_eq!(count, 0);
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
}
