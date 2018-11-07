#![allow(unused)]

fn main() {

    let names = vec!["Jane", "Jill", "Jack", "John"];

    let total_bytes = names
        .iter()
        .map(|name: &&str| name.len())
        .fold(0, |acc, len| acc + len);

    println!("total bytes: {}", total_bytes);
    assert_eq!(total_bytes, 16);
    
    use_names_for_something_else(names);


    let player_scores = [
        ("lou", 10), ("bean", 20), ("hudson", 30),
    ];

    let players: Vec<_> = player_scores
        .iter()
        .map(|(player, score)| player)
        .collect();

    println!("{:?}", players);

    let mut teams = [
        [ ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19), ],
        [ ("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17), ]
    ];

    let teams_in_score_order: Vec<_> = teams
        .iter_mut()
        .map(|team| {
            team.sort_by(|&a, &b| b.1.cmp(&a.1));
            team
        })
        .collect();

    println!("Teams: {:?}", teams_in_score_order);

    let x = vec!["Jill", "Jack", "Jane", "John"];

    let y = x
        .iter()
        .cloned()
        .take(2)
        .collect::<Vec<_>>();

    println!("{:?}", y);

}

fn use_names_for_something_else(names: Vec<&str>){

}