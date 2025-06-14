//const array of digits and their letters
const NUMBERS: [[u8; 4]; 10] = [
    [b'\0', b'\0', b'\0', b'\0'], //0
    [b'\0', b'\0', b'\0', b'\0'], //1
    [b'a', b'b', b'c', b'\0'], //2
    [b'd', b'e', b'f', b'\0'], //3
    [b'g', b'h', b'i', b'\0'], //4
    [b'j', b'k', b'l', b'\0'], //5
    [b'm', b'n', b'o', b'\0'], //6
    [b'p', b'q', b'r', b's'], //7
    [b't', b'u', b'v', b'\0'], //8
    [b'w', b'x', b'y', b'z'], //9
];
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        //handling an edge case
        if digits.is_empty(){
            return Vec::new();
        }

        let mut size = 1; //number of combinations
        let mut periods = Vec::new(); //This will make sense later
        let mut digits = digits.chars().map(|c| c.to_digit(10).unwrap() as usize); //nicer digits iterator
        for d in digits.clone(){
            //get the size and make the periods
            periods.push(size);
            size *= match d{
                7 => 4,
                9 => 4,
                _ => 3,
            }
        }

        let mut result: Vec<String> = Vec::new();
        result.resize(size, String::new());
        for (i, d) in digits.enumerate(){
            let mut j = 0;
            while j < size{ //loop for every result string
                for c in NUMBERS[d]{ //for each character
                    if c == b'\0'{ //(other than \0)
                        break;
                    }

                    let c = c as char;
                    //plop that character in for the duration of the period of that spot
                    for _ in 0..(periods[i]){
                        result[j].push(c);
                        j += 1;
                    }
                }
            }
        }

        result
    }
}