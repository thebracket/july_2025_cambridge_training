const MAX_NUMBER: usize = 100_000;

fn is_prime(n: usize) {
    if n <= 1 {
        return;
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return;
            }
        }
        println!("Found prime: {}", n);
        return;
    }
}

fn main() {
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();

    let num_cores = std::thread::available_parallelism().unwrap().get();
    let chunk_size = MAX_NUMBER / num_cores + 1;
    let chunks = candidates.chunks(chunk_size);

    std::thread::scope(|s| {
        for chunk in chunks {
            s.spawn(move || {
                for &candidate in chunk {
                    is_prime(candidate);
                }
            });
        }
    });
}
