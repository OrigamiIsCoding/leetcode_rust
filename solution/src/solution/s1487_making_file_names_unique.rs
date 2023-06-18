#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/making-file-names-unique/
pub struct Solution;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut h = std::collections::HashMap::new();

        names
            .into_iter()
            .map(|name| {
                let name = match h.get(&name).copied() {
                    Some(mut value) => loop {
                        let alias = format!("{}({})", &name, &value);
                        if h.contains_key(&alias) {
                            value += 1;
                            continue;
                        }
                        *h.get_mut(&name).unwrap() = value;
                        break alias;
                    },
                    None => name,
                };

                h.insert(name.clone(), 1);
                name
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec_into!["pes", "fifa", "gta", "pes(2019)"] as Vec<String>,
            Solution::get_folder_names(vec_into!["pes", "fifa", "gta", "pes(2019)"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec_into!["gta", "gta(1)", "gta(2)", "avalon"] as Vec<String>,
            Solution::get_folder_names(vec_into!["gta", "gta(1)", "gta", "avalon"])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec_into![
                "onepiece",
                "onepiece(1)",
                "onepiece(2)",
                "onepiece(3)",
                "onepiece(4)"
            ] as Vec<String>,
            Solution::get_folder_names(vec_into![
                "onepiece",
                "onepiece(1)",
                "onepiece(2)",
                "onepiece(3)",
                "onepiece"
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec_into!["wano", "wano(1)", "wano(2)", "wano(3)"] as Vec<String>,
            Solution::get_folder_names(vec_into!["wano", "wano", "wano", "wano"])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            vec_into!["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"] as Vec<String>,
            Solution::get_folder_names(vec_into!["kaido", "kaido(1)", "kaido", "kaido(1)"])
        );
    }
}
