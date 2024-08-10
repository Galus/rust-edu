struct CAT;
struct DOG;
trait Speak {
    fn speak(&self) {
        println!("I am god.");
    }
}
impl Speak for CAT {
    fn speak(&self) {
        println!("Meowza!");
    }
}
impl Speak for DOG {
    fn speak(&self) {
        println!("Woof!");
    }
}
fn main() {
    println!("Hello, world!");
    let meowmeow = CAT;
    meowmeow.speak();
   let sparkie = DOG;
   sparkie.speak();
}
