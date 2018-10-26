#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("blue".to_string()),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row{
       println!("{:?}", i);
    }

}