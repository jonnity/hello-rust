mod rand_util;
use crate::rand_util::pokeca_util::{cointos, d6_roll};
fn main() {
    println!("{}", cointos());
    println!("{}", d6_roll());
}
