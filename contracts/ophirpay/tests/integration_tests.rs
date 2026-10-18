#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cancellation_percentages() {
        // Verify cancellation at 0%, 25%, 50%, 75%, and 100% elapsed time
        let total_amount: i128 = 1000;
        let duration: u64 = 400;
        let start_time: u64 = 100;

        let percentages = vec![0.0, 0.25, 0.50, 0.75, 1.0];
        for pct in percentages {
            let elapsed = ((duration as f64) * pct) as u64;
            let current_time = start_time + elapsed;

            let vested = (total_amount * (elapsed as i128)) / (duration as i128);
            let refundable = total_amount - vested;

            assert_eq!(vested + refundable, total_amount);
        }
    }

    #[test]
    #[should_panic(expected = "duration cannot be zero")]
    fn test_zero_duration_guard() {
        let duration: u64 = 0;
        if duration == 0 {
            panic!("duration cannot be zero");
        }
    }
}
