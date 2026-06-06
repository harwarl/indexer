use crate::utils::contracts::IERC20::{Approval, Transfer};

pub enum DecodeResult {
    Transfer(Transfer),
    Approval(Approval),
    Unknown,
}
