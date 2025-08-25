use std::collections::HashMap;
struct Trie {
    value: char,
    is_terminal: bool,
    children: HashMap<char, Trie>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self{
            value: '\0',
            is_terminal: false,
            children: HashMap::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        if word.is_empty(){
            return;
        }

        let mut word_iter = word.chars();
        let c = word_iter.next().unwrap();
        let word: String = word_iter.collect();
        if word.is_empty() {self.is_terminal = true; return;}
        let next = word.chars().next().unwrap();
        self.children.entry(next).or_insert(Trie::new()).insert(word);
    }
    
    fn search(&self, mut word: String) -> bool {
        if word.is_empty() {return false;}
        if word.len() == 1 && self.value != '\0'{
            return self.is_terminal && word.pop().unwrap() == self.value;
        }
        let mut word_iter = word.chars();
        let c = word_iter.next().unwrap();
        let word: String = word_iter.collect();
        
        self.children.get(&c).is_some_and(|child| child.search(word))
    }
    
    fn starts_with(&self, mut prefix: String) -> bool {
        if prefix.len() == 1{
            return prefix.pop().unwrap() == self.value;
        }
        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */