use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    end: bool,
    nodes: HashMap<char, Box<Node>>,
}

struct WordDictionary {
    root: Box<Node>,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Box::new(Node {
                end: false,
                nodes: HashMap::new(),
            }),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut curr: &mut Box<Node> = &mut self.root;
        for ch in word.chars().into_iter() {
            if curr.nodes.contains_key(&ch) {
                curr = curr.nodes.get_mut(&ch).unwrap();
            } else {
                let new = Box::new(Node {
                    end: false,
                    nodes: HashMap::new(),
                });
                curr.nodes.insert(ch, new);
                curr = curr.nodes.get_mut(&ch).unwrap();
            }
        }
        curr.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut currs: Vec<&Box<Node>> = vec![&self.root];
        for (idx, ch) in word.chars().into_iter().enumerate() {
            let mut next_currs: Vec<&Box<Node>> = vec![];
            for curr in currs.iter() {
                if ch == '.' {
                    curr.nodes.iter().for_each(|x| next_currs.push(x.1));
                } else if curr.nodes.contains_key(&ch) {
                    next_currs.push(curr.nodes.get(&ch).unwrap());
                }
            }
            if next_currs.len() == 0 {
                return false;
            }
            currs = next_currs;
        }
        let mut res = false;
        for curr in currs.iter() {
            if curr.end {
                res = true;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn test_case1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word(String::from("bad"));
        word_dictionary.add_word(String::from("dad"));
        word_dictionary.add_word(String::from("mad"));
        assert_eq!(word_dictionary.search(String::from("pad")), false);
        assert_eq!(word_dictionary.search(String::from("bad")), true);
        assert_eq!(word_dictionary.search(String::from(".ad")), true);
        assert_eq!(word_dictionary.search(String::from("b..")), true);
    }

    #[test]
    fn test_case2() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word(String::from("a"));
        word_dictionary.add_word(String::from("a"));
        assert_eq!(word_dictionary.search(String::from(".")), true);
        assert_eq!(word_dictionary.search(String::from("a")), true);
        assert_eq!(word_dictionary.search(String::from("aa")), false);
        assert_eq!(word_dictionary.search(String::from("a")), true);
        assert_eq!(word_dictionary.search(String::from(".a")), false);
        assert_eq!(word_dictionary.search(String::from("a.")), false);
    }

    #[test]
    fn test_case3() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word(String::from("at"));
        word_dictionary.add_word(String::from("and"));
        word_dictionary.add_word(String::from("an"));
        word_dictionary.add_word(String::from("add"));
        assert_eq!(word_dictionary.search(String::from("a")), false);
        assert_eq!(word_dictionary.search(String::from(".at")), false);
        word_dictionary.add_word(String::from("bat"));
        assert_eq!(word_dictionary.search(String::from(".at")), true);
        assert_eq!(word_dictionary.search(String::from("an.")), true);
        assert_eq!(word_dictionary.search(String::from("a.d.")), false);
        assert_eq!(word_dictionary.search(String::from("b.")), false);
        assert_eq!(word_dictionary.search(String::from("a.d")), true);
        assert_eq!(word_dictionary.search(String::from(".")), false);
    }

    #[test]
    fn test_case4() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word(String::from("a"));
        word_dictionary.add_word(String::from("ab"));
        assert_eq!(word_dictionary.search(String::from("a")), true);
        assert_eq!(word_dictionary.search(String::from("a.")), true);
        assert_eq!(word_dictionary.search(String::from("ab")), true);
        assert_eq!(word_dictionary.search(String::from(".a")), false);
        assert_eq!(word_dictionary.search(String::from(".b")), true);
        assert_eq!(word_dictionary.search(String::from("ab.")), false);
        assert_eq!(word_dictionary.search(String::from(".")), true);
        assert_eq!(word_dictionary.search(String::from("..")), true);
    }
}

fn main() {
    println!("Hello, world!");
}
