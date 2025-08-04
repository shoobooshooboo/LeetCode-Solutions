use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        //bfs
        let mut queue = VecDeque::new(); 
        let (m, n) = (grid.len(), grid[0].len());

        //add initial rotten oranges to the queue
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 2{
                    queue.push_back((i + 1, j));
                    queue.push_back((i - 1, j));
                    queue.push_back((i, j + 1));
                    queue.push_back((i, j - 1));
                }
            }
        }

        //bfs loop will run 1 more time than the number of minutes, so start minutes one lower
        let mut minutes = -1;
        while !queue.is_empty(){
            minutes += 1;
            let len = queue.len();
            for _ in 0..len{
                let (i, j) = queue.pop_front().unwrap();
                if grid.get(i).is_none_or(|row| row.get(j).is_none_or(|&x| x != 1)){
                    continue;
                }

                grid[i][j] = 2;
                queue.push_back((i + 1, j));
                queue.push_back((i - 1, j));
                queue.push_back((i, j + 1));
                queue.push_back((i, j - 1));
            }
        }

        //if any oranges are still fresh, they will never rot
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 1{
                    return -1;
                }
            }
        }

        //max is to catch the edge case of there being no oranges at all.
        minutes.max(0)
    }
}