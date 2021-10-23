use std::collections::HashMap;

fn main() {
    let mut ht_jobnum = HashMap::new();
    ht_jobnum.insert(String::from("8L494"), String::from("qianxiaowei"));

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(&String::from("Blue"));

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("----------------------------------------------------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(60);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("----------------------------------------------------");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
