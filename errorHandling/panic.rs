fn main(){
    let mut username = String::new();
    if username.is_empty(){
        panic!("hello this is panic function");
    }
    
    println!("{}", username);
}