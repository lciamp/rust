#![allow(unused)]



struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {

    let c = CustomSmartPointer{data: String::from("Hudson")};
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before end of scope.");
    

    


}