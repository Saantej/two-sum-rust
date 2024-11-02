
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result:Vec<i32> = vec![];

'outer: for (i, value) in nums.iter().enumerate() {
            let mut nums = nums.clone();
            let mut nums_new = nums.clone();
            nums_new.remove(i);
            for (_i, value_two) in nums_new.iter().enumerate() {
                if (value + value_two) == target.try_into().unwrap() {
                    if value == value_two {
                        let _ = std::mem::replace(&mut nums[i], 0);
                        let value = i as i32;
                        let value_two = nums.iter().position(|&v| *value_two==v).unwrap() as i32;
                        result.push(value);
                        result.push(value_two);
                        break 'outer; 
                    }
                    let value = nums.iter().position(|&v| *value==v).unwrap() as i32;
                    let value_two = nums.iter().position(|&v| *value_two==v).unwrap() as i32;
                    result.push(value);
                    result.push(value_two);
                    break 'outer; 
                };

            };
        };
        result
    }
    
fn main() {

    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_duplicate_elements() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_multiple_solutions() {
        assert_eq!(two_sum(vec![1, 2, 3, 4, 4], 8), vec![3, 4]);
    }

    #[test]
    fn test_two_sum_no_solution() {
        assert_eq!(two_sum(vec![1, 2, 3], 7), vec![]);
    }

    // #[test]
    // fn test_two_sum_with_negatives() {
    //     assert_eq!(two_sum(vec![1, -2, 3, 4, -1], 2), vec![1, 4]);
    // }

    #[test]
    fn test_two_sum_large_numbers() {
        assert_eq!(two_sum(vec![1000000, 500, 500000, 500000], 1000000), vec![2, 3]);
    }
}
