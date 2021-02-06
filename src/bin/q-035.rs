fn main() {}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::*;

    if target == 0 {
        return 0;
    }

    let (mut low, mut up) = (0, nums.len() - 1);
    while low < up {
        let mid = (low + up) / 2;
        match nums[mid].cmp(&target) {
            Equal => return mid as i32,
            Less => low = mid + 1,
            Greater => {
                if mid == 0 {
                    break;
                }
                up = mid - 1;
            }
        }
    }

    if nums[low] < target {
        low as i32 + 1
    } else {
        low as i32
    }
}
