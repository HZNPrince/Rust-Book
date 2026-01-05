use core::str;

fn main(){
    enum Location {
        Point(i32),
        Range(i32, i32)
    }
    let loc1 = Location::Point(5);
    fn print_range_max(loc: &Location) {  
        // print the second field of Range, if loc is a Range
        if let Location::Range(_,num) = loc{
            println!("The second field of range is {num}");
        }
    }
    print_range_max(&loc1);
    let prince = "Prince".to_string();
    fn accepts_str(name: str){
        ()
    }
      
}