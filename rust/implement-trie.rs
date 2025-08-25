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

    fn with_value(value: char) -> Self{
        Self{
            value,
            is_terminal: false,
            children: HashMap::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        if word.is_empty(){
            self.is_terminal = true;
            return;
        }

        let mut word_iter = word.chars();
        let c = word_iter.next().unwrap();
        let word: String = word_iter.collect();
        self.children.entry(c).or_insert(Trie::with_value(c)).insert(word);
    }
    
    fn search(&self, mut word: String) -> bool {
        println!("{}", self.value);
        if word.is_empty() {return self.is_terminal;}
        let mut word_iter = word.chars();
        let c = word_iter.next().unwrap();
        let word: String = word_iter.collect();

        self.children.get(&c).is_some_and(|child| child.search(word))
    }
    
    fn starts_with(&self, mut word: String) -> bool {
        if word.is_empty() {return true;}
        let mut word_iter = word.chars();
        let c = word_iter.next().unwrap();
        let word: String = word_iter.collect();
        
        self.children.get(&c).is_some_and(|child| child.starts_with(word))
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */