#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix[0].len();
        let mut matrix = [
            vec![vec![0; m + 1]],
            matrix
                .into_iter()
                .map(|row| {
                    [
                        vec![0],
                        row.into_iter()
                            .map(|v| if v == '1' { 1 } else { 0 })
                            .collect(),
                    ]
                    .concat()
                })
                .collect(),
        ]
        .concat();

        let mut ans = 0;

        for i in 1..=n {
            for j in 1..=m {
                matrix[i][j] += matrix[i][j - 1] + matrix[i - 1][j] - matrix[i - 1][j - 1];
            }
        }

        println!("{}", matrix[2][2]);
        for x1 in 1..=n {
            for x2 in x1..=n {
                let h = x2 - x1 + 1;
                let w = ans / h;
                for y1 in 1..=m {
                    for y2 in y1 + w - 1..=m {
                        let w = y2 - y1 + 1;
                        if h * w
                            == matrix[x2][y2] - matrix[x2][y1 - 1] - matrix[x1 - 1][y2]
                                + matrix[x1 - 1][y1 - 1]
                        {
                            ans = ans.max(h * w);
                        }
                    }
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(mat![
                ['1', '0', '1', '0', '0'],
                ['1', '0', '1', '1', '1'],
                ['1', '1', '1', '1', '1'],
                ['1', '0', '0', '1', '0']
            ])
        );
    }
}
