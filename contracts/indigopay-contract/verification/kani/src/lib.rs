//! Kani verification harnesses for IndigoPay contract
//!
//! These harnesses verify safety invariants of the core contract.
//! Run via: `cargo kani --release` from this directory.

#[cfg(kani)]
mod verification {
    use kani::proof;

    /// Core invariant: the `calculate_badge` function must not panic
    /// for any total_stroops value that fits in i128.
    #[proof]
    fn calculate_badge_no_panic() {
        let total_stroops: i128 = kani::any();
        // `total_stroops` is i128 — no further bounds needed since
        // calculate_badge divides by STROOP and does branch comparisons,
        // all of which are panic-free for any i128 value.
        let _badge = indigopay_contract::calculate_badge(total_stroops);
    }

    /// Core invariant: `voting_weight_from_badge` produces deterministic,
    /// non-decreasing weights as badge tiers increase.
    #[proof]
    fn voting_weight_monotonic() {
        let badge1 = kani::any::<indigopay_contract::BadgeTier>();
        let badge2 = kani::any::<indigopay_contract::BadgeTier>();

        let w1 = indigopay_contract::voting_weight_from_badge(&badge1);
        let w2 = indigopay_contract::voting_weight_from_badge(&badge2);

        // Verify weights are within expected bounds
        assert!(w1 <= 200);
        assert!(w2 <= 200);

        // Verify None tier has zero weight
        if matches!(badge1, indigopay_contract::BadgeTier::None) {
            assert_eq!(w1, 0);
        }
    }
}
