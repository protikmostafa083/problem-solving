use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // expect() - if unable to read, it will provide the given error message

    let n:i32 = input.trim().parse().expect("invalid input");

    // craeting a loop to iterate
    for _ in 0..n{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        let s = s.trim();

        if s.len() > 10{
            println!("{}{}{}",&s[0..1],s.len() - 2, &s[s.len() - 1..])
        }else{
            println!("{}",s)
        }
    }
}