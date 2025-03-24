#[allow(dead_code)]

//Declaration of functions in rust
pub fn area_of(x: i32, y: i32) -> i32 {
    // -> denotes the return type 
    return x * y;
}

pub fn print_distance((x,y): (f32, f32)) {
    println!(
        "Distance to the origin is {}",
        (x + y).sqrt()
    );
}

pub fn sum() {
    let mut sum = 0;

    //for loop iterating throught integers 7 to 23
    for i in 7..=23{
        sum +=i
        //adding all the elements in this range
    }


    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 1;

    //while loop
    while x < 500 {
        count += 1;
        x *= 2;
    } 

    println!("You can double x {} times until x is larger than 500", count);
}

