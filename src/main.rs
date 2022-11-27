fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let l_pass = reverse_hash::retrieve_data("8f005cb95f2ca99877c2d4db3e33861c", 24);

    let elapsed = now.elapsed();
    println!("Word: {}", l_pass);
    println!("Elapsed: {:.2?}", elapsed);
}
