fn factorial(n: u32) -> u32 {
    if n==0 {
        1
    } 
    else {
        n * factorial(n -1)
    }
}

pub fn exec() {
    let num = 5; 
    println!("Fact of N {} is {} ", num, factorial(num));
}
