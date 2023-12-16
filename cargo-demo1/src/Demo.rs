use std::cmp::Ordering;
use std::mem;


#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U
}


impl<T,U> Point<T,U>{

    fn x(&self)-> &T {
        &self.x
    }

    fn mix_up<V,W>(self, other:Point<V,W>) -> Point<T,W>{
        Point{
            x: self.x,
            y:other.y
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

async fn demo(b:&mut String) -> String {

    let r1 = Rectangle {
        width : 100,
        length: 100
    };


    println!("The area of the rectangle is {:?} square pixels.",r1 );
    println!("The area of the rectangle is {:#?} square pixels.",r1);

    println!("The area of the rectangle is: {}",r1.area());
    println!("The area of the rectangle is: {}",r1.width);

    if r1.width() {
        println!("The rectangle has a nonzero width; it is {}", r.width);
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

    let number_list = vec![32,50,25,100,65];
    let mut largest = get_largest(number_list);

    println!("Larget Number {} ",largest);

    let char_list = vec!['a','b','c'];
    let mut larget_char = get_largest(char_list);

    println!("Larget char {}",larget_char);


    let p1 = Point{x:5,y:10};

    let p2 = Point{x:0.2,y:10.1};
    let p3 = Point{x:1,y:10.2};
    let p1 = Point{x: "Hello",y:12};
    let p2 = Point{x: 21,y:"Hello"};
    println!("{:?}",p1.mix_up(p2));

    let mut a :String = String::from("Hello");

    let mut s1 = b;
    {
        let _s2  = s1;

        s1 = _s2;
    }
    a.push_str(s1);
    return a;
}


fn demo1() {
    println!("Guess the number!");

    let _secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {_secret_number}");

    loop {
        println!("The Please input you guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");

        println!("Your guess is {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }


        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");

        let s1 : String = String::from("Hello");


        println!("{s1}");

        let mut b = String::from("Hello");
        
    }



    
}
fn fizzBuzz(){
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Reading from stdin failed");

    let _x: u32 =  input.trim().parse().expect("Invalid");

    if _x%3==0 && _x%5==0  {
        println!("FizzBuzz");
    }    
    else if _x%3==0 {
        println!("Fizz");
    }
    else if _x%5==0 {
        println!("Buzz");
    }
    else{
        println!("No")
    }
}

fn get_largest<T:PartialOrd+Copy>(number_list: Vec<T>) -> T {
    let mut largest:T = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}