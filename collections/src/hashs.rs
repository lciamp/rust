#![allow(unused)]

use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_scores = vec![("Blue".to_string(), 10), 
                           ("Yellow".to_string(), 50)];

    for i in team_scores.iter() {
        println!("{:?}", i);
    }
   
    let test: HashMap<_,_> = team_scores.into_iter().collect();

    println!("test: {:?}", test);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("scores: {:?}", scores);

    
    for (k, v) in scores.iter() {
        println!("{} - {}", k, v);
    }

    // OWNERSHIP:
    let field_name = String::from("Favroite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    // field_name and field_value are invalid now
    //println!("{}", field_name);
    //println!("{}", field_value);

    let mut scores = HashMap::new();

    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    println!("score: {:?}", score.unwrap());










}