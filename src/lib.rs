// Silence some warnings .
#![allow(dead_code, unused_mut, unused_variables)]

pub fn greet() {
    println!("Hi");
}
//pub makes it public. To access throughout the project

pub fn simple_type() {
    println!("----------------------- FROM LIB ----------------------");
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference(coords.0, coords.1);

    let coords_array: [f32; 2] = [coords.0, coords.1];
    println!("-------------------------");
    print_array(coords_array);

    println!("-------------------------");
    let series: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
    //7 i32 types
    ding(series[6]);

    println!("-------------------------"); 
    let mess: ([i32; 2], f64, [(bool, i32); 2], i32, &str) =
    ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    //Mess is a tuple, tuple can be 12 char long to be printable
    // Tuples can be used as function arguments and as return values.
    on_off(mess.2[1].0);
    //Want to send true from Tuple
    
    println!("-------------------------");
    print_distance(coords)

}



fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

fn print_distance((x,y): (f32, f32)) {
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

pub fn count(arg: String) {
    let mut count = 0;

    //conditional loop
    loop {
        count += 1;
        println!("Executed {} times ", count);

        if count >= 8 {
            break; 
        }
    }

    print!("End of flow"); 
}

