use std::collections::HashMap;
// let mut my_map = HashMap::new();
/*EXPLAINATION
    The error you're encountering in Rust is due to attempting to declare a variable at the module level directly inside src/main.rs without enclosing it in a function or any other item. Rust does not allow variable declarations directly in the module scope. Variables must be declared within a function, struct, or enum, etc
 */

fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("name", "John");
    my_map.insert("age", "30");

    // get value
    // match my_map.get("age") {
    //     Some(value) => println!("age is {}", value),
    //     None => println!("Name not found"),
    // }

    // Assuming my_map is a HashMap<&str, &str>
    println!("age of {} is {}", my_map.get("name").unwrap_or_else(|| &("Name not found")) , my_map.get("age").unwrap_or_else(|| &("age not found")));

    /*
        19 | ... my_map.get("age").unwrap_or_else(|| "age not found"));
           |                                         ^^^^^^^^^^^^^^^ expected `&&str`, found `&str`
           |    SO I ADDED &("age not found") TO MAKE IT WORK

     */

    //remove value
    my_map.remove("age");
    println!("after removing");
    println!("age of {} is {}", my_map.get("name").unwrap_or_else(|| &("not found")) , my_map.get("age").unwrap_or_else(|| &("not found")));
}
