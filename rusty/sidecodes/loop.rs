use std::io;
fn main(){
    /*
    // none is of type Option<T>
    let i = None;
    /*
        The if let syntax is a convenient way to unpack values 
        from types that implement the std::ops::Pattern trait, such as Option and Result
     */
    //  which means it checks if i is an Option containing the value 5.
    if let Some(5) = i {
        println!("i is 5");
    } else {
        println!("i is not 5");
    }
     */

    // while loop
    /*
    
    while count < 11{
        println!("count is {}", count);
        count += 1;
    }
     
    let mut input = String::new();
    while input.trim() != "end" {
        input.clear();
        println!("Enter a value: ");
        io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {}", input);
    }
    
    for i in (0..=11).rev() {
        print!("i is {}\n", i);
    }
    

    for i in vec![1, 7, 3, 9, 5] {
        println!("i is {}", i);
    }
    */
    //for match 

    loop{
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num = input.trim().parse::<i32>().expect("Not a number");
        match num {
            // notice the commas instead of semicolons
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            4 => println!("Four"),
            5 => println!("Five"),
            // _ is a catch all. also notice the block for multiple line code
            _ => {
                println!("Not a number between 1 and 5");
                break;
            }


        }
    }
}