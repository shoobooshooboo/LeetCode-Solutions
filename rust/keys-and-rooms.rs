use std::collections::HashSet;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        //simple, intuitive hashset solution.

        //set the size of keys to the number of rooms so no resizing has to be done.
        //if we want to optimize space instead of speed, then we wouldn't do this.
        let mut keys = HashSet::with_capacity(rooms.len());
        //we start in room 0 so basically like we have the key to it.
        keys.insert(0);
        Self::get_keys(&rooms, 0, &mut keys);
        keys.len() == rooms.len()
    }

    fn get_keys(rooms: &Vec<Vec<i32>>, room_number: i32, mut keys: &mut HashSet<i32>){
        //for each key in this room, if it is a new key, go to that room and gather all the keys.
        for &k in &rooms[room_number as usize]{
            if keys.insert(k){
                Self::get_keys(rooms, k, keys);
            }
        }
    }
}