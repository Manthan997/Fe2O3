fn main(){
    // example for assigning a variable
    let my_var = 4;

    let result = if my_var == 5 {
        "my_var is 5"
    } else {
        "my_var is not 5"
    };

    // you can redefine variable is Rust
    let my_var = if my_var < 5 {"damn"} else {"not damn"};

    println!("Result: {}\nMy_var: {}", result, my_var);

}