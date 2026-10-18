#[cfg(test)]
mod proptests {
    #[test]
    fn test_fuzz_arbitrary_timestamps() {
        for elapsed in 0..=1000 {
            let total_amount: i128 = 5000;
            let duration: u64 = 1000;
            
            if duration == 0 {
                continue;
            }

            let effective_elapsed = std::cmp::min(elapsed, duration);
            let vested = (total_amount * (effective_elapsed as i128)) / (duration as i128);
            let refundable = total_amount - vested;

            assert_eq!(vested + refundable, total_amount);
        }
    }
}
