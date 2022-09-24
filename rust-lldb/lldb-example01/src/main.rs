fn binary_search(nums: Vec<i32>,target:i32) -> i32 {
    let mut size = nums.len();

    if size == 0 {
        return -1;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        if nums[mid] <= target {
            base = mid;
        }
        size = size - half;
    }
    if nums[base] == target {
        return base as i32;
    }
    return -1;
}


fn main() {
    assert_eq!(binary_search(vec![1,4,7,10,16,19], 10),3);
}
