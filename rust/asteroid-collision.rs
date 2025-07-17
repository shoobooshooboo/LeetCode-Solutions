use std::cmp::Ordering::*;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        //use a Vec like a stack to manage the remaining asteroids.
        let mut result = Vec::<i32>::with_capacity(asteroids.len());
        'outer: for a in asteroids{
            // if a left-bound asteroid successfully joins the stack,
            // it will never be hit and right-bound asteroids
            // cannot cause a collision upon their arrival
            while result.last().is_some_and(|b| *b > 0) && a < 0{
                //manage asteroid collision
                match result.last().unwrap().cmp(&(a.abs())){
                    Less => {result.pop();},
                    Equal => {result.pop(); continue 'outer;},
                    Greater => continue 'outer,
                }
            }
            result.push(a);
        }

        result
    }
}