// // Maximum Subarray
// fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let length = nums.len();
//     if length == 0 { return nums[0]; }
    
//     let mut largest = nums[0];
//     let mut real_largest = nums[0];
//     let mut i = 1;
    
//     while i < length {
//         let current = nums[i];
//         let curr_plus_largest = current + largest;
        
//         if current < curr_plus_largest {
//             largest = curr_plus_largest;
//         } else {
//             largest = current;
//         }
        
//         if real_largest < largest {
//             real_largest = largest;
//         }
        
//         i += 1;
//     }
    
//     real_largest
// }

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut lg = nums[0];
    let mut real_lg = nums[0];
    for num in nums.into_iter().skip(1) {
        let curr_sum = num + lg;
        if num > curr_sum {
            lg = num;
        } else {
            lg = curr_sum;
        }
        if real_lg < lg {
            real_lg = lg;
        }
    }
    real_lg
}

// fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let mut lg = nums[0];
//     let mut real_lg = nums[0];
//     nums.into_iter().skip(1).for_each(|n| {
//         let curr = n + lg;
//         if n > curr { 
//             lg = n; 
//         } else { 
//             lg = curr; 
//         }
//         if real_lg < lg { real_lg = lg; }
//     });
//     real_lg
// }