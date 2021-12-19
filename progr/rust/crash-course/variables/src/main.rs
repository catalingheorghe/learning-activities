const STARTING_MISSILES: i32 = 8;
const READY_MISSILES: i32 = 2;

fn main() {
    //let mut missiles = STARTING_MISSILES;
    //let ready: i32 = READY_MISSILES;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_MISSILES);
    
    println!("Firing {} of my {} missiles!", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    //println!("{} missiles left", missiles - ready); // warn about mut
}