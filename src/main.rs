pub fn main(){
    //defines the main func, entry pt
    //pub means public, dont need to specify as a func is by default public
    //The main func doesnot return any value

    let x = 99;
    //In rust vars are immutable by default, means x cannot be changed unless declared explicitly as 'mut'
    //here the type of x is automatically inferred as i32 (32 bit int) by default

    let is_even = is_even(x); //  returns a bool value

    if is_even { //means true
        print!("{} is even", x);
        //Thsi prints '99 is even' 
        //print! is a macro in rust used for printing without any new line
        //'{}' is a placeholder for the value of x
    } else {
        print! ("{} is odd", x);
    }

    pub fn is_even(x: i32) -> bool {
        return x % 2 == 0;
    }
    //'-> bool' the func returns a bool value
    //if the remainder after the module returns 0 then exp evaluates to true  

    // fn main() {
    //     let mut num: i8 = 124;
    //     for i in 0..100 {
    //         num += 127;
    //     }
    //     print!("Number: {}", num)
    // }
    // func overflow!!! as num of type i8

    let greetings = String::from("Hello World");
    //'string::from this creats a string from the string literal'
    //A string in rust is heap allocated, growable string size, whereas &str is fixed size
    //'String::from' is a method that converts a string literal into a String type, which allows for more flexible string manipulation

    //Heap Allocation: The String type stores the data on the heap, which allows it to grow dynamically.
    //The string literal gets copied into the heap memory, and 'greetings' holds reference to that memory

    //Printing the string:
    //The '!' in println is used to show that this is amacro func
    //{}- palceholder also used for fomatting in Rust


    println!("{}", greetings)

    //println!("{}", greetings.chars().nth(1000))


}