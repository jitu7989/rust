

// use std::collections::btree_map::Values;
// use std::io;
// use std::io::Empty;
use std::mem;

fn main() {


    let r1 = Rectangle {
        width : 100,
        length: 100
    };


    println!("The area of the rectangle is {:?} square pixels.",r1 );
    println!("The area of the rectangle is {:#?} square pixels.",r1);

    println!("The area of the rectangle is: {}",r1.area());
    println!("The area of the rectangle is: {}",r1.width);

    if r1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let xs: [i32; 5] = [1,2,3,4,5];
    
    let ys: [i32; 500] = [ -1; 500];



    println!("First element of the array: {}",xs[0]);
    println!("First element of the array: {}",xs[1]);

    println!("Lenght of the array: {}" ,xs.len());

    println!("Array Occupies {} bytes",mem::size_of_val(&xs));

    println!("Borrow the whole array as slice");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32;0]=[];
    assert_eq!(&empty_array,&[]);
    assert_eq!(&empty_array,&[][..]);

    for i in 0..xs.len()+1{
        match  xs.get(i) {
            Some(xval) => println!("{}: {}",i,xval),
            None => println!("Slow down! {} is too far!",i) 
        }
    }
    
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn width(&self) -> bool {
        self.width > 0 
    }
    
}


fn analyze_slice(slice: &[i32]){

    println!("First Element of the slice: {}",slice[0]);
    println!("The Slice has {} elements",slice.len());

}
/*
fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
*/