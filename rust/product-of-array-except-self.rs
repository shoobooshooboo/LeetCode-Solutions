impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        //two pointer approach using prefix and suffix products.
        let mut products = vec![1;nums.len()];
        let (mut i, mut j) = (0, nums.len() - 1);

        let (mut prefix, mut suffix) = (1, 1);
        while i < nums.len(){
            products[i] *= prefix;
            products[j] *= suffix;
            prefix *= nums[i];
            suffix *= nums[j];

            i += 1;
            j -= 1;
        }

        products
    }
}