pub fn main() {
    conditionals();
}

fn conditionals() {
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    if making_event {
        println!("You can attend the event!");
    } else {
        println!("You cannot attend the event.");
    }
}
