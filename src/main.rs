use rand::seq::SliceRandom;

fn main() {
    let my_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        print!("{}", my_letters.choose(&mut rng).unwrap());
    }
}
