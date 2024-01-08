#[derive(Debug)]
#[allow(dead_code)]
struct Object {
    width:u32,
    height:u32
}

//methods
impl Object {

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn show(&self) {
        println!("area is {} {} {}",self.width, self.height, self.area())
    }
}
// Related Functions
impl Object {
    fn new(width:u32,height:u32)-> Object {
        Object {
            width,
            height
        }
    }
}
#[derive(Debug)]

enum Direction{
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point)
}
#[derive(Debug)]

enum Keys{
    Upkey(String),
    Downkey(String),
    Leftkey(String),
    Rightkey(String)
}
impl Direction {
    fn match_direction(&self)->Keys{
        match *self {
            Direction::Up(_) => Keys::Upkey(String::from("Pressed w")),
            Direction::Down(_) => Keys::Downkey(String::from("Pressed s")),
            Direction::Left(_) => Keys::Leftkey(String::from("Pressed a")),
            Direction::Right(_) => Keys::Rightkey(String::from("Pressed d")),

        }
    }
}

impl Keys{
    fn destruct(&self) -> &String{
        match *self {
            Keys::Upkey(ref s) => s,
            Keys::Downkey(ref s) => s,
            Keys::Leftkey(ref s) => s,
            Keys::Rightkey(ref s) => s,
            
        }
    }
}
#[derive(Debug)]

struct Point{
    x:i32,
    y:i32
}
fn main(){
    let u = Direction :: Up(Point{x:1,y:0});
    let k = u.match_direction();
    let x = k.destruct();
    println!("{:?}",k);

    println!("{}",x);

}