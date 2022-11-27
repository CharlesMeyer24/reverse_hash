use std::sync::Arc;
use std::sync::Mutex;
#[cfg(debug_assertions)]
use std::time::Instant;

fn retrieve_data_impl(
    p_depth_limit: i8,
    mut p_depth: i8,
    p_str: &mut String,
    p_mutex: &Arc<Mutex<String>>,
    p_hash: &str,
) -> bool {
    if p_depth == p_depth_limit - 1 {
        // compute digest from hash
        let l_digest = md5::compute(&p_str);

        // Compare it to the hash we expect
        if format!("{:X}", l_digest).to_lowercase().eq(p_hash) {
            // Bingo !
            #[cfg(debug_assertions)]
            println!("Password is {}", p_str);
            let mut l_return = p_mutex.lock().unwrap();
            *l_return = (*p_str).clone();
            return true;
        }
    }

    p_depth += 1;
    if p_depth >= p_depth_limit {
        return false;
    }

    for l_printable in ' '..='~' {
        p_str.push(l_printable);
        if retrieve_data_impl(p_depth_limit, p_depth, p_str, p_mutex, p_hash) {
            return true;
        }
        p_str.pop();
    }

    false
}

pub fn retrieve_data(p_hash: &str, p_num_threads: i64) -> String {
    let mut p_depth_limit = 1;
    let l_mutex = Arc::new(Mutex::new(String::new()));
    let l_chars: Vec<char> = (' '..='~').collect();

    loop {
        let l_threads_vec: Vec<&[char]> = l_chars
            .chunks((l_chars.len() as f64 / p_num_threads as f64).ceil() as usize)
            .collect();

        #[cfg(debug_assertions)]
        let l_now = Instant::now();

        // Blocked untill all threads are joined
        crossbeam::scope(|scope| {
            for l_thread_vec in l_threads_vec {
                let l_copy_mutex = Arc::clone(&l_mutex);
                scope.spawn(move || {
                    let mut p_str = String::new();

                    for l_printable in l_thread_vec {
                        p_str.push(l_printable.clone());

                        if retrieve_data_impl(p_depth_limit, 0, &mut p_str, &l_copy_mutex, p_hash) {
                            return;
                        }
                        p_str.pop();
                    }
                });
            }
        });

        // Print time elpased for current depth
        #[cfg(debug_assertions)]
        println!(
            "Depth limit : {} finished in {:.2?}",
            p_depth_limit,
            l_now.elapsed()
        );
        p_depth_limit += 1;

        // Check if we have found a solution
        {
            let l_return = l_mutex.lock().unwrap();
            if !(*l_return).is_empty() {
                return (*l_return).clone();
            }
        }
    }
}
