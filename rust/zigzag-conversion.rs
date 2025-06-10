use std::collections::HashMap;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        //map each row to the string in the row
        let mut row_map = HashMap::new();
        for i in 0..num_rows{
            row_map.insert(i, String::new());
        }

        //walk through the string and collect the chars
        let mut row = 0;
        let mut inc = false;
        for c in s.chars(){
            println!("{row}");
            row_map.get_mut(&row).unwrap().push(c);
            if row == 0 || row == num_rows - 1{
                inc = !inc;
            }
            if num_rows != 1{
                row += if inc {1} else {-1};
            }
        }

        //place all the rows into a result string
        let mut result = String::new();
        for i in 0..num_rows{
            result.push_str(row_map.get(&i).unwrap());
        }

        result
    }
}