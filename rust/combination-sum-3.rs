impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        //simple recursive backtracking solution
        let mut result = Vec::new();
        //addends holds the current path at any given moment
        let mut addends = Vec::with_capacity(k as usize);
        Self::combination_sum3_rec(k, n, 1, &mut addends, &mut result);

        result
    }

    fn combination_sum3_rec(k: i32, n: i32, min: i32, addends: &mut Vec<i32>, output: &mut Vec<Vec<i32>>){
        //handles the last addend
        if k == 1{
            if n <= 9 && n >= min{
                addends.push(n);
                output.push(addends.clone());
                addends.pop();
            }
            return;
        }

        //checks all remaining possible numbers
        for i in min..=9{
            //if the current number is too large, we can just call it quits
            if i >= n{
                break;
            }

            //otherwise, push the current number and do the recursive call.
            addends.push(i);
            //reduce k and n and increase min in the recursive call.
            Self::combination_sum3_rec(k - 1, n - i, i + 1, addends, output);
            //backtrack
            addends.pop();
        }
    }
}