impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        //simple dfs solution
        //keep track of all visisted
        let mut visited = Vec::with_capacity(is_connected.len());
        visited.resize(is_connected.len(), false);
        let mut count = 0;
        for i in 0..is_connected.len(){
            if !visited[i]{
                //each dfs will uncover a new province. 
                //therefor, the number of dfs-es is the number of provinces
                Self::dfs(&is_connected, i, &mut visited);
                count += 1;
            }
        }

        count
    }

    fn dfs(is_connected: &Vec<Vec<i32>>, i: usize, visited: &mut Vec<bool>){
        visited[i] = true;
        for j in 0..is_connected[i].len(){
            //same sorta deal as the keys and rooms one.
            if is_connected[i][j] == 1 && !visited[j]{
                Self::dfs(is_connected, j, visited);
            }
        }
    }
}