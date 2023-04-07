use std::io;        // calling the standard input output library

fn main(){
    let mut input = String::new(); // create a mutable string and an empty variable
    io::stdin().read_line(&mut input).unwrap();
    // stdin() mrthod is used to handle standard input stream
    // read_line() - reads the line from standard input stream
    // &mut - refers that the input in mutable which is the INPUT variable
    // unwrap() - returns the value of the result or panics if the value is an error

    let w:i32 = input.trim().parse().unwrap();

    // creates a immutable i32 variable names w
    // trim() - removes any whitespaces
    // parse() - parses the string into some kind of number
    // unwrap() - returns the value of the result or panics if the value is an error

    if w%2 == 0 && w>2{
        println!("YES");
    }
    else{
        println!("NO");
    }
}
