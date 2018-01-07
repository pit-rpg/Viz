extern crate uuid;
use uuid::Uuid;

fn main(){
    let a = Uuid::new_v4();

    println!("{:?}", 11);
    println!("{:?}", a);
}
