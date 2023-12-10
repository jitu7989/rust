fn main(){

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");

    println!("{:?}", ("Hello","World","All") );

    let people: &str = "some people";

    println!("{people} {text}",text="must live" );

    // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");




    println!("{}",assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !"));
    println!("{}",assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!"));
    // println!("{}",assert_eq!(format!("Hello {:^5}!", "x"),  "Hello   x  !"));
    // println!("{}",assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!"));
}