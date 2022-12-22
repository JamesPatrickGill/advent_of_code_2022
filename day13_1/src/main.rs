use std::cmp::Ordering;

use serde_json::Value;

fn main() {
    let input = include_str!("../input.txt");
    let pairs: Vec<(Value, Value)> = input
        .split_terminator("\n\n")
        .map(|pair_str| pair_str.split_terminator("\n").collect())
        .map(|pairs: Vec<&str>| {
            (
                serde_json::from_str(pairs[0]).unwrap(),
                serde_json::from_str(pairs[1]).unwrap(),
            )
        })
        .collect();

    let mut ordered = vec![];
    for (idx, pair) in pairs.into_iter().enumerate() {
        let res = compare(pair.0, pair.1);
        if res == InOrder::YES {
            ordered.push(idx + 1);
        }
    }
    println!("{}", ordered.iter().sum::<usize>())
}

#[derive(Debug, PartialEq)]
enum InOrder {
    YES,
    NO,
    DUNNO,
}

fn compare(left: Value, right: Value) -> InOrder {
    if left.is_i64() && right.is_i64() {
        let left_num = left.as_i64().unwrap();
        let right_num = right.as_i64().unwrap();
        if left_num < right_num {
            return InOrder::YES;
        } else if left_num > right_num {
            return InOrder::NO;
        }
        return InOrder::DUNNO;
    }

    if left.is_i64() && right.is_array() {
        let left_arr: Value =
            serde_json::from_str(&format!("[{}]", left.as_i64().unwrap())).unwrap();
        return compare(left_arr, right);
    }

    if right.is_i64() && left.is_array() {
        let right_arr: Value =
            serde_json::from_str(&format!("[{}]", right.as_i64().unwrap())).unwrap();
        return compare(left, right_arr);
    }

    if left.is_array() && right.is_array() {
        let mut left_arr = left.as_array().unwrap().to_owned();
        let mut right_arr = right.as_array().unwrap().to_owned();

        left_arr = left_arr.into_iter().rev().collect();
        right_arr = right_arr.into_iter().rev().collect();
        loop {
            let l_val = left_arr.pop();
            let r_val = right_arr.pop();

            if l_val.is_none() && r_val.is_none() {
                return InOrder::DUNNO;
            }

            if l_val.is_some() && r_val.is_none() {
                return InOrder::NO;
            }

            if l_val.is_none() && r_val.is_some() {
                return InOrder::YES;
            }

            let res = compare(l_val.unwrap(), r_val.unwrap());
            if res == InOrder::DUNNO {
                continue;
            }
            return res;
        }
    }

    InOrder::DUNNO
}
