#[cfg(test)]
mod tests {
    use hash_brute_force::{hash::hash_of_num, main_logic::compute_result};

    #[test]
    fn can_gen_hash() {
        hash_of_num(1);
    }

    #[test]
    fn normal_work_first() {
        let v: Vec<(usize, String)> = vec![
            (
                4163,
                "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
            ),
            (
                11848,
                "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000",
            ),
            (
                12843,
                "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000",
            ),
            (
                13467,
                "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000",
            ),
            (
                20215,
                "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000",
            ),
            (
                28892,
                "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000",
            ),
        ]
        .into_iter()
        .map(|(num, hash)| (num, hash.to_string()))
        .collect();
        assert_eq!(v, compute_result(3, 6));
    }

    #[test]
    fn normal_work_second() {
        let v: Vec<(usize, String)> = vec![
            (
                828028,
                "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000",
            ),
            (
                2513638,
                "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000",
            ),
            (
                3063274,
                "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000",
            ),
        ]
        .into_iter()
        .map(|(num, hash)| (num, hash.to_string()))
        .collect();
        assert_eq!(v, compute_result(5, 3))
    }
}
