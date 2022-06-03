pub use solana_program::log::*;
use solana_program::{account_info::AccountInfo, msg};

#[macro_export]
macro_rules! debug_msg {
    ($msg:expr) => {
        if cfg!(feature = "debug") {
            $crate::log::gth_log($msg);
            $crate::log::gth_log_compute_units();
        }
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        if cfg!(feature = "debug") {
            $crate::log::gth_log_64(
                $arg1 as u64,
                $arg2 as u64,
                $arg3 as u64,
                $arg4 as u64,
                $arg5 as u64,
            );
            $crate::log::gth_log_compute_units();
        }
    };
    ($($arg:tt)*) => (if cfg!(feature = "debug") {
        $crate::log::gth_log(&format!($($arg)*));
        $crate::log::gth_log_compute_units();
    });
}

#[allow(dead_code)]
pub fn gth_log_params_short(accounts: &[AccountInfo], data: &[u8]) {
    msg!("Accounts:");
    for account in accounts.iter() {
        account.key.log();
    }
    msg!("Instruction data length:");
    msg!(0, 0, 0, 0, data.len());
}
