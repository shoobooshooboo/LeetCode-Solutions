use std::collections::HashMap;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        //hard coding numerals
        let numerals: HashMap<i32, &str> = HashMap::from([
            (0, ""),
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
            (10, "X"),
            (20, "XX"),
            (30, "XXX"),
            (40, "XL"),
            (50, "L"),
            (60, "LX"),
            (70, "LXX"),
            (80, "LXXX"),
            (90, "XC"),
            (100, "C"),
            (200, "CC"),
            (300, "CCC"),
            (400, "CD"),
            (500, "D"),
            (600, "DC"),
            (700, "DCC"),
            (800, "DCCC"),
            (900, "CM"),
            (1000, "M"),
            (2000, "MM"),
            (3000, "MMM"),
        ]);

        let mut result = String::new();
        result.insert_str(0, numerals[&(num % 10)]);
        num -= num % 10;
        println!("{num}");
        result.insert_str(0, numerals[&(num % 100)]);
        num -= num % 100;
        println!("{num}");
        result.insert_str(0, numerals[&(num % 1000)]);
        num -= num % 1000;
        println!("{num}");
        result.insert_str(0, numerals[&num]);

        result
    }
}