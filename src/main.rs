use RaceNG;
fn main() {
    loop {
        let result = RaceNG::race(3, 5);
        println!("RNG says {:?}", result);
    }
}