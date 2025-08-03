use std::collections::VecDeque;
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        //easy bfs. literally did a harder version of this problem year 1 of college.
        let mut queue = VecDeque::new();
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        //(x, y), steps
        queue.push_back((entrance, 0));
        //working with options instead of default "null" values is the rust way.
        let mut min_steps: Option<i32> = None;
        while !queue.is_empty(){
            //check out all the new spaces to check
            let len = queue.len();
            for _ in 0..len{
                let (coord, steps) = queue.pop_front().unwrap();
                //is this a valid spot?
                if maze.get(coord.0).is_none_or(|row| row.get(coord.1).is_none_or(|&c| c == '+')){
                    continue;
                //is this an exit?
                }else if steps != 0 && (coord.0 == 0 || coord.0 == maze.len() - 1 || coord.1 == 0 || coord.1 == maze[0].len() - 1) {
                    if let Some(min) = min_steps.take(){
                        min_steps = Some(min.min(steps));
                    }else{
                        min_steps = Some(steps);
                    }
                    continue;
                }
                //don't come back to this spot
                maze[coord.0][coord.1] = '+';

                //get all the next spots
                queue.push_back(((coord.0 + 1, coord.1), steps + 1));
                queue.push_back(((coord.0 - 1, coord.1), steps + 1));
                queue.push_back(((coord.0, coord.1 + 1), steps + 1));
                queue.push_back(((coord.0, coord.1 - 1), steps + 1));
            }
        }

        min_steps.unwrap_or(-1)
    }
}