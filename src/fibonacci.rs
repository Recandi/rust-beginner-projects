pub fn fib(i: u32) -> u32{
    if i == 1{
        return 0;
    }else if i == 2{
        return 1;
    }
    let mut f1 = 0;
    let mut f2 = 1;
    for _e in 3..i+1{
        let t = f1 + f2;
        f1 = f2;
        f2 = t;
    }
    return f2;
}

pub fn fib_recursive(i: u32) -> u32{
    if i == 1{
        return 0;
    }else if i == 2{
        return 1;
    }else{
        return fib_recursive(i - 1) + fib_recursive(i - 2);
    }
}