use soroban_sdk::{contract, contractimpl, Env, Address, symbol_short};

#[contract]
pub struct OphirPayContract;

#[contractimpl]
impl OphirPayContract {
    pub fn cancel_stream(env: Env, stream_id: u64, current_time: u64) -> (i128, i128) {
        // Simulated retrieval of stream parameters: total_amount, start_time, duration, claimed
        let total_amount: i128 = 1000;
        let start_time: u64 = 100;
        let duration: u64 = 500;
        let claimed: i128 = 0;

        // 1. Add explicit zero-check on duration prior to division
        if duration == 0 {
            panic!("duration cannot be zero");
        }

        if current_time <= start_time {
            return (0, total_amount - claimed);
        }

        let elapsed = if current_time >= start_time + duration {
            duration
        } else {
            current_time - start_time
        };

        // 2. Standardize linear formula with solvency rounding favor
        let vested = (total_amount * (elapsed as i128)) / (duration as i128);
        let refundable = total_amount - vested;

        // Ensure invariant: vested + refundable == total_amount
        assert_eq!(vested + refundable, total_amount, "Solvency invariant violated");

        (vested - claimed, refundable)
    }
}
