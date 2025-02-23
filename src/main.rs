mod app_1;
use app_1::a;
use app_1::PERSON;

fn main() {
    println!("Hello, world!");
    let result = a(4294967295, 2);
    println!("Result: {}", result);

    println!("{}", i32::max_value().to_string());

    let my_struct = create_object();


    println!("Age: {}", my_struct.age);
    println!("Name: {}", my_struct.name);

    println!("{}", PERSON);

    let status = true;

    let _current_sum = if status {
        1;
    }
    else {
        2;
    };




    let mut n = 1;
    loop{
        println!("n = {}", n);
        n = n + 1;
        if n == 10{
            break;
        }
    }
    println!("Конец программы"); 


    let mut num = 1;
    let result = loop
    {
        if num == 4 { 
            break num * 2;
        }
        num = num + 1;
    };
    println!("result = {}", result);    // result = 8   


    for num in 1..6
    {
        println!("num = {}", num);
    }
    println!("Конец программы"); 

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



