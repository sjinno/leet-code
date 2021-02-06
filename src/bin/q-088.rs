fn main() {}

#[rustfmt::skip]
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    use std::cmp::Ordering::*;

    let (m, n) = (m as usize, n as usize);
    match (m, n) {
        (0, _) => {
            for i in 0..n {
                nums1[i] = nums2[i];
            }
        },
        (_, 0) => (), 
        (_, _) => {
            let (mut p, mut q) = (m - 1, n - 1);
            let mut idx = m + n - 1;
            
            loop {
                match nums1[p].cmp(&nums2[q]) {
                    Less | Equal => { 
                        nums1[idx] = nums2[q];
                        idx -= 1;
                        if q == 0 { break; }
                        q -= 1;
                    },
                    Greater => { 
                        nums1[idx] = nums1[p];
                        idx -= 1;
                        if p == 0 {
                            while q != 0 {
                                nums1[idx] = nums2[q];
                                q -= 1;
                                idx -= 1;
                            }
                            nums1[idx] = nums2[q];
                            break;
                        }
                        p -= 1;
                    },
                }
            }
        }
    }   
}
