#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
impl Rectangle{
    pub fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add(2, 2));
        assert_eq!(4, add(2, 2));
    }

    // #[test]
    // fn failing_test(){
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller(){

        let larger = Rectangle{
            width:7,
            height:7
        };
        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));


    }

}
