use rand::Rng;
use rand::distributions::Uniform;

fn main() {
    println!("Get your own color!");

    let mut rng = rand::thread_rng();
    let values = Uniform::from(0..256);

    let output = (0..3)
        .fold(String::new(), |acc, _| acc + &format!("{:02x}", rng.sample(&values)));

    println!("Your color is #{output}");
}
