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

fn main(){
    let o = Object {
        width: 10,
        height: 20
    };

    o.show();
    let obj = Object :: new(2,3);
    obj.show();
}