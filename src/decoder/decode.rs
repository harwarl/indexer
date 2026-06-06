use alloy::rpc::types::Log;

use crate::{
    types::DecodeResult,
    utils::contracts::IERC20::{Approval, Transfer},
};

pub fn try_decode(log: &Log) -> DecodeResult {
    if let Ok(event) = log.log_decode::<Transfer>() {
        return DecodeResult::Transfer(event.inner.data);
    }

    if let Ok(event) = log.log_decode::<Approval>() {
        return DecodeResult::Approval(event.inner.data);
    }

    return DecodeResult::Unknown;
}
