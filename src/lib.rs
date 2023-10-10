pub mod main_logic {
    use std::sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    };
    use threadpool::ThreadPool;

    use crate::hash::hash_of_num;

    pub fn compute_result(null: usize, first: usize) -> Vec<(usize, String)> {
        let counter = Arc::new(Mutex::new(0));
        let pool = ThreadPool::new(num_cpus::get()); // ThreadPool by number of cores
        let (tx, rx) = channel();

        let end_str = (0..null).map(|_| '0').collect::<String>(); // generate String to check hashes
        let mut num = 0; // first number for compute hash

        while need_to_contunue(first, Arc::clone(&counter)) {
            hash_of_num_in_thread(
                &pool,
                num,
                end_str.clone(),
                first,
                tx.clone(),
                Arc::clone(&counter),
            );
            num += 1;
        }
        rx.iter().take(first).collect::<Vec<(usize, String)>>()
    }

    fn hash_of_num_in_thread(
        pool: &ThreadPool,
        num: usize,
        end_str: String,
        first: usize,
        tx: Sender<(usize, String)>,
        counter: Arc<Mutex<usize>>,
    ) {
        // start new thread
        pool.execute(move || {
            if need_to_contunue(first, Arc::clone(&counter)) {
                let hash = hash_of_num(num);
                if hash.ends_with(end_str.as_str()) {
                    // hash condition check
                    tx.send((num, hash)).expect("Channel is close");
                    *counter.lock().unwrap() += 1;
                }
            }
        });
    }

    fn need_to_contunue(first: usize, counter: Arc<Mutex<usize>>) -> bool {
        // check COUNT for completed tasks
        *counter.lock().unwrap() != first
    }
}

pub mod hash {
    use data_encoding::HEXLOWER;
    use ring::digest;

    pub fn hash_of_num(num: usize) -> String {
        // compute hash and translate to String
        HEXLOWER.encode(digest::digest(&digest::SHA256, num.to_string().as_bytes()).as_ref())
    }
}

pub mod cmd {
    use clap::{self, command, Parser};

    #[derive(Parser)]
    #[command(name = "hash-brute-force")]
    #[command(author = "9i5BCrUcNX5NmT")]
    #[command(version)]
    #[command(about = "Selects hashes of numbers by specified parameters", long_about = None)]
    struct Cli {
        /// number of zeros at the end of hashes found
        #[arg(short, long)]
        null: usize,

        /// number of hashes to be found
        #[arg(short, long)]
        first: usize,
    }

    pub fn collect_args() -> (usize, usize) {
        let cli = Cli::parse();

        (cli.null, cli.first)
    }
}
