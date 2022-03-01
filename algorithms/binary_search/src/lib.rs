use std::cmp;

pub fn clamp(value: i32, low: i32, high: i32) -> i32 {
    let clamped = cmp::max(low, value);
    let clamped = cmp::min(clamped, high);
    clamped
}

pub fn binary_search<F>(evaluate: F, low: i32, high: i32, target: i32) -> (i32, i32)
where
    F: Fn(i32) -> i32,
{
    let mut low_index = low;
    let mut high_index = high;

    loop {
        let index = clamp((low_index + high_index) / 2, low, high);
        let value = evaluate(index);
        if value == target {
            return (index, -1);
        }

        if (high_index - low_index) <= 1 {
            let low_value = evaluate(low_index);

            if low_value == target {
                return (low_index, -1);
            }

            if low_value > target {
                return (-1, clamp(low_index - 1, low, high));
            }

            let high_value = evaluate(high_index);

            if high_value == target {
                return (high_index, -1);
            }

            if high_value < target {
                return (-1, high_index + 1);
            }

            return (-1, low_index + 1);
        }

        if value < target {
            low_index = index;
        } else {
            high_index = index;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search;

    fn validate(nums: &Vec<i32>, target: i32, expected: (i32, i32)) {
        println!(
            " nums: {:?}, target: {}, expected: {:?}",
            nums, target, expected
        );
        let answer = binary_search(
            |index| nums[index as usize],
            0,
            (nums.len() - 1) as i32,
            target,
        );
        assert_eq!(answer, expected);
    }

    #[test]
    fn small() {
        let nums = vec![-1, 1, 3, 5];
        validate(&nums, -2, (-1, 0));
        validate(&nums, 2, (-1, 2));
        validate(&nums, 6, (-1, 4));
    }

    #[test]
    fn edge_cases() {
        validate(&vec![5], -2, (-1, 0));
        validate(&vec![5], 6, (-1, 1));
    }
}
