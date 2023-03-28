// 在数组中找出指定值，存在返回索引，否则返回None
fn search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(search(&nums1, 4), Some(3));
    assert_eq!(search(&nums1, 7), Some(6));
    assert_eq!(search(&nums1, 10), None);

    let nums2 = vec![34, 56, 67, 90, 101, 200, 356, 521, 845, 999];
    assert_eq!(search(&nums2, 101), Some(4));
    assert_eq!(search(&nums2, 999), Some(9));
    assert_eq!(search(&nums2, 1000), None);
}
