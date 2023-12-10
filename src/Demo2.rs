fn main(){

    println!("{} days",31);

    println!("'{1}' 1 Arguments and then '{0}' 0 Argument ", "0 Arg" , "1 Arg");
    println!("{subject} {marks} {student}",subject="Maths" , marks="30", student ="Chekon" );

    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C


    // You can right-justify text with a specified width. 
    println!("{number:>5}", number=1);
    // println!("{<:5}{number:>5}", number=1);

    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000

    println!("{number:0>width$}", number=1, width=6);

    /*  
     println!("{0} 0 {1} 1","Bond");
     Compilation error
     println!("{0} 0 {1} 1","Bond",)
                      ^
        = note: positional arguments are zero-based
     */

     println!("{0} 0 {1} 1","Bond","James");


    //  #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

   

}