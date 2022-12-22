use std::cmp::Ordering;

use serde_json::Value;

fn main() {
    let input = include_str!("../input.txt");
    let mut lines: Vec<Value> = input
        .split_terminator("\n")
        .filter(|line| line.len() != 0)
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    lines.push(serde_json::from_str("[[2]]").unwrap());
    lines.push(serde_json::from_str("[[6]]").unwrap());

    lines.sort_by(|a, b| {
        let res = compare(a.to_owned(), b.to_owned());
        if res == InOrder::YES {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        };
    });

    let mut res = 1;
    for (idx, item) in lines.into_iter().enumerate() {
        let str_item = serde_json::to_string(&item).unwrap();
        if str_item == "[[2]]" || str_item == "[[6]]" {
            res *= idx + 1
        }
    }
    println!("{res:?}");
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
