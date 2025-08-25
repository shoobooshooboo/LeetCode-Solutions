impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        //levenshtein distance algorithm.
        //change strings to byte arrays for easy indexing
        let start = word1.into_bytes();
        let end = word2.into_bytes();
        //the dp matrix
        let mut min_operations: Vec<Vec<i32>> = vec![vec![0; end.len() + 1]; start.len() + 1];

        //set the upper and leftmost edges of the matrix
        for i in 0..min_operations.len(){
            min_operations[i][0] = i as i32;
        }
        for j in 0..min_operations[0].len(){
            min_operations[0][j] = j as i32;
        }

        for i in 1..min_operations.len(){
            for j in 1..min_operations[0].len(){
                // substition_cost is 0 if substitution is not needed
                let substitution_cost = if start[i - 1] == end[j - 1] {0} else {1};

                //minimum value of the possible operations
                min_operations[i][j] =
                    (min_operations[i - 1][j - 1] + substitution_cost).min(
                    (min_operations[i - 1][j] + 1).min(
                    (min_operations[i][j - 1] + 1)
                    ));
            }
        }

        //give 'em the last thing in the matrix
        min_operations.pop().unwrap().pop().unwrap()
    }
}