use std::io;
use io::Write;
///Prints nicely formatted multiplication table to the console
pub fn multiply(){

    print!("Please enter size of table: ");
    
    //Get tablesize as input and parse to number + 1
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading from CL went Wrong");
    input = input.trim().to_string();
    let mut x : u32= input.parse().expect("NaN");
    x = x + 1;

    //find biggest number, cast to string to get necessary spacing for formatting
    let space = (x * x).to_string().len() + 1;
    
    //vertical index offset
    print!("{:3}", "");
    for i in 0..x{
        print!("{:s$}", i, s=space);
    }
    
    println!("");
    for i in 0..x{
        print!("{:3}", i);
        for e in 0..x{
            print!{"{:s$}", i * e, s=space};
        }
        println!("");
    }
}