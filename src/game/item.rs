use serde::Deserialize;
use pcre::Pcre;

#[derive(Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub hidden: Option<bool>
}

pub fn get_article(word: &String) -> String {
    let a = "a".to_string();
    let an = "an".to_string();
    let mut an_re = Pcre::compile(r"^([aeiou]|ho).*").unwrap();
    if an_re.matches(word).count() > 0 {
        an
    }
    else {
        a
    }
}

impl Item {

    pub fn items_to_text(items: &Vec<String>) -> Option<String> {
        if items.len() == 0 {
            None
        }
        else if items.len() == 1 {
            Some(get_article(&items[0]) + " " + &items[0].clone())
        }
        else {
            let mut s = String::new();
            for i in 0..items.len() - 1 {
                s = s + &get_article(&items[i]) + " " + &items[i] + ", ";
            }
            let last = &items.last().unwrap();
            s = s + "and " + &get_article(last) + " " + last;
            Some(s)
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::game::item::*;

    #[test]
    fn test_get_article() {
        let a = "a".to_string();
        let an = "an".to_string();
        assert_eq!(get_article(&"foo".to_string()), a);
        assert_eq!(get_article(&"abacus".to_string()), an);
        assert_eq!(get_article(&"yellow".to_string()), a);
        assert_eq!(get_article(&"hour".to_string()), an);
    }
}