use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    // immutable references to elements...
    let x = vec![100, 32, 57];
    for i in &x {
        println!("{}", i);
    }

    // mutable references for change elements..
    let mut v = vec![20, 30, 40];
    for i in &mut v {
        *i += 2;
    }
    println!("{:?}", v);

    // vec can contain elements of different types using enums...
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    // String is also a collection... and have the same operations...
    let s = String::new();
    println!("{}", s);
    let data = "string literal";
    let s = data.to_string();
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    s1.push('p');
    println!("{}", s1);

    // concat with + operator...
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // concat with format! macro...
    let s4 = String::from("s4");
    let s5 = String::from("s5");
    let s6 = String::from("s6");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s7);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    hash_map();
    hash_map_from_iterators();

    let map = new_hash_map();
    let letter_name = String::from("a");
    let score = map.get(&letter_name);
    match score {
        Some(value) => println!("score: {}", value),
        None => println!("no score!"),
    }
    iterate_over_map();
    updating_map();
    only_insert_value_if_the_key_has_no_value();
    updating_a_value_based_on_the_old_value();
    median_and_mode_values();
    pig_latin("character");
    pig_latin("amazon");
}

fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}

fn hash_map_from_iterators() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}

fn new_hash_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("a"), 20);
    scores.insert(String::from("b"), 80);
    scores
}

fn iterate_over_map() {
    let scores = HashMap::from([("Green", 10), ("Black", 20)]);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Grey"), 15);
    scores.insert(String::from("Grey"), 75);

    println!("{:?}", scores);
}

fn only_insert_value_if_the_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn updating_a_value_based_on_the_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn median_and_mode_values() {
    let mut vec = vec![4, 9, 2, 6, 1, 9, 2, 1, 2];
    vec.sort();
    println!("median value: {:?}", vec[vec.len() / 2]);

    let mut map = HashMap::new();
    let mut count_occurs_most = 0;
    let mut value_occurs_most = &vec[0];
    for v in &vec {
        let count = map.entry(v).or_insert(0);
        *count += 1;
        if *count > count_occurs_most {
            value_occurs_most = v;
            count_occurs_most = *count;
        }
    }
    println!("map {:?}", map);
    println!("value occurs most: {}", value_occurs_most);
}

//first -> irst-fay
//today -> oday-tay
fn pig_latin(string: &str) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let first_char = string.chars().nth(0).unwrap();
    let result = match vowels.iter().find(move |&&v| v == first_char) {
        Some(_ch) => format!("{}-{}", string, String::from("hay")),
        None => format!(
            "{}-{}{}",
            &string[1..string.len()],
            &string[0..1],
            String::from("ay")
        ),
    };
    println!("{}", result);
}
