//! The Mint that represents the native token

/// There are 10^9 weis in one GTH
pub const DECIMALS: u8 = 9;

// The Mint for native GTH Token accounts
solana_program::declare_id!("So11111111111111111111111111111111111111112");

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::native_token::*;

    #[test]
    fn test_decimals() {
        assert!(
            (weis_to_gth(42) - crate::amount_to_ui_amount(42, DECIMALS)).abs() < f64::EPSILON
        );
        assert_eq!(
            gth_to_weis(42.),
            crate::ui_amount_to_amount(42., DECIMALS)
        );
    }
}
