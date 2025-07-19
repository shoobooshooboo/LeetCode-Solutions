impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        //senate into bytes for easier indexing.
        let mut senate = senate.into_bytes();
        //r_bans as bans cast by Radiant. D bans are bans cast by Dire.
        let (mut r_bans, mut d_bans) = (0, 0);
        let mut i = 0;
        loop{
            //at the end of each loop, check the current state.
            if i >= senate.len() {
                //if one party is eliminated, the other wins
                if !senate.contains(&b'R'){
                    return "Dire".to_string();
                }else if !senate.contains(&b'D'){
                    return "Radiant".to_string();
                }
                //otherwise, loop through again
                i = 0;
            }
            //either ban the current senator or add to their ban counter.
            if senate[i] == b'R'{
                if d_bans >= 1{
                    senate[i] = b'L';
                    d_bans -= 1;
                }else{
                    r_bans += 1;
                }
            }else if senate[i] == b'D'{
                if r_bans >= 1{
                    senate[i] = b'L';
                    r_bans -= 1;
                }else{
                    d_bans += 1;
                }
            }

            i += 1;
        }
        "Error".to_string()
    }
}