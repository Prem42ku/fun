fn main(){
    let x: i32 = 453;
    let xc: Box<i32> = Box::new(123);
    let answer: i32 = x + *xc;
    println!("{:?}",answer);
}

