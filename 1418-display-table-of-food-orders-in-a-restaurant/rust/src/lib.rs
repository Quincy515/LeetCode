use std::collections::BTreeMap;
use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 按字典顺序排列的餐品名称 {"Apple": 0, "Banner": 0}
        let mut food_map = BTreeMap::new();
        orders.iter().for_each(|v| {
            food_map.entry(&v[2]).or_insert(0);
        });

        // 给餐品名称依次标上下标 {"Apple": 0, "Banner": 1}
        food_map
            .iter_mut()
            .enumerate()
            .for_each(|(i, (_, x))| *x = i);

        // 餐桌订购的相应餐品数量，key 是餐桌序号 {1: [2, 0], 12: [0, 3]}
        let mut order_map = BTreeMap::new();
        orders.iter().for_each(|v| {
            order_map
                .entry(i32::from_str(&v[1]).unwrap())
                .or_insert_with(|| vec![0; food_map.len()])[food_map[&v[2]]] += 1;
        });

        // 标题：["Table", "Apple", "Banner"]
        let mut title = vec!["Table".to_string()];
        title.append(
            &mut food_map
                .iter()
                .map(|(&name, _)| name.to_string())
                .collect::<Vec<String>>(),
        );

        // 拼接返回结果数组
        let mut result = vec![title];
        order_map.iter().for_each(|(id, v)| {
            let mut order = vec![id.to_string()];
            order.append(&mut v.iter().map(|x| x.to_string()).collect::<Vec<String>>());
            result.push(order);
        });
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::display_table(vec![
                vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
                vec![
                    "Corina".to_string(),
                    "10".to_string(),
                    "Beef Burrito".to_string()
                ],
                vec![
                    "David".to_string(),
                    "3".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
                vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
                vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Beef Burrito".to_string(),
                    "Ceviche".to_string(),
                    "Fried Chicken".to_string(),
                    "Water".to_string()
                ],
                vec![
                    "3".to_string(),
                    "0".to_string(),
                    "2".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                vec![
                    "5".to_string(),
                    "0".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                    "1".to_string()
                ],
                vec![
                    "10".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                    "0".to_string()
                ]
            ]
        );
        assert_eq!(
            Solution::display_table(vec![
                vec![
                    "James".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Ratesh".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Amadeus".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Adam".to_string(),
                    "1".to_string(),
                    "Canadian Waffles".to_string()
                ],
                vec![
                    "Brianna".to_string(),
                    "1".to_string(),
                    "Canadian Waffles".to_string()
                ]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Canadian Waffles".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec!["1".to_string(), "2".to_string(), "0".to_string()],
                vec!["12".to_string(), "0".to_string(), "3".to_string()]
            ]
        );
        assert_eq!(
            Solution::display_table(vec![
                vec![
                    "Laura".to_string(),
                    "2".to_string(),
                    "Bean Burrito".to_string()
                ],
                vec![
                    "Jhon".to_string(),
                    "2".to_string(),
                    "Beef Burrito".to_string()
                ],
                vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Bean Burrito".to_string(),
                    "Beef Burrito".to_string(),
                    "Soda".to_string()
                ],
                vec![
                    "2".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                    "1".to_string()
                ]
            ]
        );
    }
}
