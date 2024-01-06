use std::io;
use rand::Rng;
fn send_btc(){
    println!("Lets send BTC");
}

fn recieve_btc(){
    println!("Lets recieve BTC");
    let number = rand::thread_rng().gen_range(1,10);
    println!("Recived BTC is {}",number);

}

fn ganja(){
    println!("invalid jojo");
}

fn send_recieveBTC(){

    println!("\n Letts have fun with BTC \n");

    println!("Do you wanna send (s) or recieve (r) bitcoin");

    let mut command = String::new();
    io::stdin().read_line(&mut command);
    if command.trim().eq("s"){
        send_btc()
    }
    else if command.trim().eq("r")
    {
        recieve_btc()
    }
    else{
        ganja()
    }
}

fn main() {
    send_recieveBTC()
}
