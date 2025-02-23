fn main() {
    println!("Hello, world!");
    let result = a(1, 2);
    println!("Result: {}", result);

    println!("{}", i32::max_value().to_string());

    let my_struct = create_object();
    println!("Field1: {}", my_struct.field1);
    println!("Field2: {}", my_struct.field2);

}


fn a(a: i32, b: i32) -> i32 {
    return a + b;
}

struct MyStruct {
    field1: i32,
    field2: String,
}

fn create_object() -> MyStruct {
    MyStruct {
        field1: 42,
        field2: String::from("Hello"),
    }
}

