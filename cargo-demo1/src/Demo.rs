use std::cmp::Ordering;
async fn demo(b:&mut String) -> String {

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