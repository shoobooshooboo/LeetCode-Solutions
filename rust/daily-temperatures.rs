impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut days = vec![0; temperatures.len()];
        //monotonic stack of the indexes of the temperatures in non-increasing order of temperature
        let mut mono_stack = Vec::with_capacity(temperatures.len());
        for i in 0..temperatures.len(){
            //regular monotonic stack operations
            if temperatures[i] <= temperatures[*mono_stack.last().unwrap_or(&i)]{
                mono_stack.push(i);
            }else{
                while let Some(j) = mono_stack.pop(){
                    if temperatures[i] <= temperatures[j]{
                        mono_stack.push(j);
                        break;
                    }
                    //every day popped from the stack gets assigned a value in the days array
                    days[j] = (i - j) as i32;
                }
                mono_stack.push(i);
            }
        }
        days
    }
}