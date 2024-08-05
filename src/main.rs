use std::collections::HashMap;
use std::io;
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
        value: None,
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
            println!("Median of the list {list:?} is {median}");
            match max.value {
                Some(val) => println!("Most occuring value in the list is {val}"),
                None => println!(" Something vent wrong, no values in the list"),
            }
        }
        None => println!("Something went wrong"),
    };
}

fn is_vowel(c: &char) -> bool {
    matches!(c, 'a' | 'e' | 'o' | 'i' | 'u' | 'A' | 'E' | 'O' | 'I' | 'U')
}

fn is_consonant(c: &char) -> bool {
    c.is_alphabetic() & !is_vowel(c)
}

fn exercise_two() {
    // Convert strings to pig latin. The first consonant of each word
    // is moved to the end of the word and ay is added, so first becomes
    // irst-fay. Words that start with a vowel have hay added to the end
    // instead (apple becomes apple-hay). Keep in mind the details about
    // UTF-8 encoding!

    let word = String::from("apple");
    let mut pig_latin_word: Option<String> = None;

    for (idx, c) in word.chars().enumerate() {
        if is_consonant(&c) {
            let s = &word[idx + 1..];
            pig_latin_word = Some(format!("{s}-{c}ay"));
            break;
        } else if (idx == 0) & is_vowel(&c) {
            pig_latin_word = Some(format!("{word}-hay"));
            break;
        }
    }
    match pig_latin_word {
        Some(pig_latin) => println!("The word: {word} converted to pig latin: {pig_latin}"),
        None => println!("Unable to convert the word: {word} to pig latin"),
    }
}

enum Command {
    Add { department: String, name: String },
    View { department: String },
    Quit,
    Invalid,
}

fn to_cmd(line: &str) -> Command {
    let list: Vec<&str> = line.split(' ').collect();
    let cmd = list.first().expect("Index out of bound").trim().to_string();
    match list.len() {
        2 => {
            if cmd == "View" {
                let dep = list.get(1).expect("Index out of bound").trim();
                Command::View {
                    department: dep.to_string(),
                }
            } else {
                Command::Invalid
            }
        }
        4 => {
            if cmd == "Add" {
                let department = list.get(3).expect("Index out of bound").trim().to_string();
                let name = list.get(1).expect("Index out of bound").trim().to_string();
                Command::Add { department, name }
            } else {
                Command::Invalid
            }
        }
        1 => {
            if cmd == "Quit" {
                Command::Quit
            } else {
                Command::Invalid
            }
        }
        _ => Command::Invalid,
    }
}

fn exercise_three() {
    // Using a hash map and vectors, create a text interface to allow a
    // user to add employee names to a department in a company; for example,
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user
    // retrieve a list of all people in a department or all people in the company
    // by department, sorted alphabetically.

    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    println!("Enter a command");
    let mut run_program = true;
    while run_program {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match to_cmd(&input) {
            Command::Add { department, name } => {
                let v = hash_map.entry(department).or_default();
                v.push(name);
                println!("{hash_map:?}");
            }
            Command::View { department } => {
                let list = hash_map.get(&department);

                match list {
                    Some(l) => {
                        let mut all = l.clone();
                        all.sort();
                        println!("All people in {department}: {all:?}")
                    }
                    None => println!("There is no department named {department}"),
                }
            }
            Command::Quit => run_program = false,
            Command::Invalid => {
                println!("Invalid command!")
            }
        }
    }
}

fn main() {
    exercise_one();
    exercise_two();
    exercise_three();
}
