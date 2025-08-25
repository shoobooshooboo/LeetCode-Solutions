use std::collections::HashSet;
impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        //naive solution. n^2 time (kinda. it's slow. that's all that really matters.)
        //sort the array
        products.sort();

        //now just go through the loop and snatch the first 3 that match the current prefix
        let mut result: Vec<Vec<String>> = Vec::new();
        for i in 0..search_word.len(){
            let prefix = &search_word[0..=i];
            let mut row = Vec::new();
            for word in products.iter(){
                if word.starts_with(prefix){
                    row.push(word.clone());
                }
                if row.len() >= 3{
                    break;
                }
            }
            result.push(row);
        }
        result
    }
}