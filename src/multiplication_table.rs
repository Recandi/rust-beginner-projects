pub fn multiply(){
    print!(" ");
    for i in 0..10{
        print!("{:4}", i);
    }
    println!("");
    for i in 0..10{
        print!("{}", i);
        for e in 0..10{
            print!{"{:4}", i * e};
        }
        println!("");
    }
}