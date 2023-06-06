pub struct Solution;


use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut h = HashMap::new();
        
        for row in grid.iter() {
            let row = row
                .iter()
                .map(|x| x.to_string())
                .fold(String::new(), |mut acc, x| {
                    acc.push(',');
                    acc.push_str(&x);
                    acc
                });
            *(h.entry(row).or_insert(0)) += 1;
        }
        
        let (n, m) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for j in 0..m {
            let mut line = String::new();
            for i in 0..n {
                line.push(',');
                line.push_str(grid[i][j].to_string().as_str());
            }
            ans += h.get(&line).unwrap_or(&0);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::equal_pairs(mat![[3, 2, 1], [1, 7, 6], [2, 7, 7]])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::equal_pairs(mat![[3, 1, 2, 2], [1, 4, 4, 5], [2, 4, 2, 2], [2, 4, 2, 2]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::equal_pairs(mat![[11, 1], [1, 11]]));
    }
}
