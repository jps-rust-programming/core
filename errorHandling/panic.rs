
struct Name {
    name: String,
    age: i32,
}
#[allow(unused_mut)]
fn main() {
    let mut username = String::new();
    username.push_str("Jai Pal");
    let name = Name {
        name: "jaipal".to_string(),
        age: 30,
    };

    

    if username.is_empty() {
        panic!("Hello! This is panic function");
    }

    println!("{}", username);
    println!("{} - {}",name.name, name.age);
}
