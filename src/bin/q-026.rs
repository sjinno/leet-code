fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let length = nums.len();
    if length == 0 { return 0; }
    if length == 1 { return 1; }
    
    let mut idx = length - 2;
    let mut to_delete = length - 1;
    
    while idx != 0 {
        if nums[idx] == nums[to_delete] {
            nums.remove(to_delete);
        }
        idx -= 1;
        to_delete -= 1;
    }
    
    if nums[idx] == nums[to_delete] {
        nums.remove(to_delete);
    }
    
    nums.len() as i32
}


// fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let mut length = nums.len();
//     if length == 0 { return 0; }
//     if length == 1 { return 1; }
    
//     for i in (0..length-1).rev() {
//         if nums[i] == nums[i+1] {
//             nums.remove(i+1);
//             length -= 1;
//         }
//     }
    
//     length as i32
// }