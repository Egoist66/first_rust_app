mod app_1;
use app_1::a;

fn main() {
    println!("Hello, world!");
    let result = a(4294967295, 2);
    println!("Result: {}", result);

    println!("{}", i32::max_value().to_string());

    let my_struct = create_object();
    println!("Age: {}", my_struct.age);
    println!("Name: {}", my_struct.name);

}




struct MyStruct {
    age: i32,
    name: String,
}

/// Creates a new instance of MyStruct with default values
fn create_object() -> MyStruct {
    MyStruct {
        age: 42,
        name: String::from("Farid"),
    }
}

