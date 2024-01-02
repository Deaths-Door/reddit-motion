use std::ops::Range;

fn main() {
    let z = 20..30;
    let mut current_pos = 27;
    let max = 25;

    println!("old position: {}", current_pos);
    ensure_in_bounds(z, &mut current_pos, max);

    println!("new position: {}", current_pos);
}


#[test]
fn a(){
    let z = 1..10;
    let mut current_pos = 3;
    let max = 5;

    ensure_in_bounds(z, &mut current_pos, max);

    println!("pos={current_pos}");
    assert!(current_pos == 3);
}

#[test]
fn b(){
    let z = 20..30;
    let mut current_pos = 27;
    let max = 25;

    ensure_in_bounds(z, &mut current_pos, max);

    println!("pos={current_pos}");
    assert!(current_pos == 2);
}

#[test]
fn c() {
    let z = 0..50;
    let mut current_pos = 35;
    let max = 50;

    ensure_in_bounds(z, &mut current_pos, max);

    println!("pos={current_pos}");
    assert!(current_pos == 35);
}

#[test]
fn d() {
    let z = 0..80;
    let mut current_pos = 65;
    let max = 75;

    ensure_in_bounds(z, &mut current_pos, max);

    println!("pos={current_pos}");
    assert!(current_pos == 65);
}


fn ensure_in_bounds(
    mut z : Range<u32>,
    current_pos : &mut u32,
    max : u32
) {
    let range_z_default = |z : &Range<u32>| z.end + z.start + 1;
    let mut range_z = range_z_default(&z);

    if range_z < max {
        z = (z.start + range_z)..(z.end + range_z);
        range_z = range_z_default(&z);
    }

    if range_z > max {
        let q = range_z / max;
        let r = range_z % max;
        println!("q={q};r={r}");
        
        for _ in 0..q-1 {
            // maybe get 'chunk' range of it , but thats for later
            println!("doing smth");
        }

        *current_pos = r+1;
    }
}
