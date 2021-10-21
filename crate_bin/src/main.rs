#[allow(unused_variables)]

use crate_demo::hitron_sz;

fn main() {
    let fancy = hitron_sz::rd_bu::new("Fancy", 80330, "5课");

    println!("{}, {}", fancy.username, fancy.usernumber);
    match fancy.department {
        hitron_sz::HitronBU::RdSubBuName(value) => println!("{}", value),
        _ => println!("Other Department")
    }
}
