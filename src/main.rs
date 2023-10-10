use hash_brute_force::{cmd::collect_args, main_logic::compute_result};

fn main() {
    let (null, find) = collect_args(); // arguments from the condition

    for (num, hash) in compute_result(null, find) {
        println!("{}, \"{}\"", num, hash);
    }
}
