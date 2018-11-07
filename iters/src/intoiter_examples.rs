#![allow(unused)]



fn main() {

    let v = vec!((String::from("Hudson"), 6));
    let names = get_names(v);

    assert_eq!(names, ["Hudson"]);
    println!("{:?}", names);
    
    
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let x = x
        .clone()
        .into_iter()
        .take(2)
        .collect::<Vec<_>>();

    println!("{:?}", x);
    

}

fn get_names(v: Vec<(String, usize)>) -> Vec<String> {
    v.into_iter()
        .map(|(name, _score)| name)
        .collect()
}

