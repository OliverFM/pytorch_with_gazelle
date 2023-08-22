use num::bigint::BigInt;
use num::FromPrimitive;

fn main() {
    let bigint: BigInt = FromPrimitive::from_u64(2).unwrap();
    println!("hello this is rust, with my chosen number, {}", bigint);
}
