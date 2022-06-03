use solana_program::{account_info::AccountInfo, clock::Epoch, pubkey::Pubkey};

#[derive(Clone)]
pub struct NativeAccountData {
    pub key: Pubkey,
    pub weis: u64,
    pub data: Vec<u8>,
    pub program_id: Pubkey,
    pub is_signer: bool,
}

impl NativeAccountData {
    pub fn new(size: usize, program_id: Pubkey) -> Self {
        Self {
            key: Pubkey::new_unique(),
            weis: 0,
            data: vec![0; size],
            program_id,
            is_signer: false,
        }
    }

    pub fn new_from_account_info(account_info: &AccountInfo) -> Self {
        Self {
            key: *account_info.key,
            weis: **account_info.weis.borrow(),
            data: account_info.data.borrow().to_vec(),
            program_id: *account_info.owner,
            is_signer: account_info.is_signer,
        }
    }

    pub fn as_account_info(&mut self) -> AccountInfo {
        AccountInfo::new(
            &self.key,
            self.is_signer,
            false,
            &mut self.weis,
            &mut self.data[..],
            &self.program_id,
            false,
            Epoch::default(),
        )
    }
}
