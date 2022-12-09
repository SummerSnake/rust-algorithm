/**
 * 多数元素（力扣 169）
 *
 * <1> 给定一个大小为 n 的数组 nums ，返回其中的多数元素;
 * <2> 多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素;
 * <3> 可以假设数组是非空的，并且给定的数组总是存在多数元素。
 */
use std::collections::HashMap;

/**
 * @desc HashMap 辅助
 */
pub fn majority_element_01(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut k = 0;
    let mut v = 0;
    for (key, value) in map.iter() {
        if value > &v {
            v = *value;
            k = *key;
        }
    }

    k
}

/**
 * @desc 因为一定存在 多数元素，且 多数元素 大于n/2，所以直接排序输出n/2位置的数即可
 */
pub fn majority_element_02(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    nums[nums.len() / 2]
}

/**
 * @desc 摩尔投票法
 *
 * <1> 对抗阶段：分属两个候选人的票数进行两两对抗抵消;
 * <2> 计数阶段：计算对抗结果中最后留下的候选人票数是否有效。
 */
pub fn majority_element_03(nums: Vec<i32>) -> i32 {
    let mut major = 0;
    let mut count = 0;

    for num in nums {
        if count == 0 {
            major = num;
        }

        if num == major {
            count += 1;
        } else {
            count -= 1;
        }
    }

    major
}
