fn division(x:f64,y:f64) -> Option<f64>{
    if y == 0.0{
        None
    }
    else{
        Some(x/y)
    }
}
fn main(){
    let res = division(9.8,0.0);
    match res {
        Some(x) => println!("{:.10}",x),
        None => println!("cannot divide b 0"),
    }
}