use std::collections::HashMap;
#[derive(Debug)]

struct Max {
    count: i32,
    value: Option<i32>,
}

fn exercise_one() {
    // Given a list of integers, use a vector and return the median 
    // (when sorted, the value in the middle position) and mode (the 
    // value that occurs most often; a hash map will be helpful here) 
    // of the list.

    let mut list = vec![2, 87, 102, 2, 16];
    let mut hash_map = HashMap::new();
    for val in &list {
        let count = hash_map.entry(*val).or_insert(0);
        *count += 1;
    }

    let mut max = Max {
        count: 0,
        value: None
    };

 
    for (k, count) in &hash_map {
        if *count > max.count {
            max.count = *count;
            max.value = Some(*k)
        }
    }

    list.sort();
    let len = list.len();
    let median_index = len / 2;
    let median_result = list.get(median_index);

    match median_result {
        Some(value) => { 
            let median = value;
            println!("Median of the list {:?} is {median}", list);
            match max.value {
                Some(val) => println!("Most occuring value in the list is {val}"),
                None => println!(" Something vent wrong, no values in the list")
            }
        }
        None => println!("Something went wrong")
    };

}

fn main() {
    exercise_one();
}
