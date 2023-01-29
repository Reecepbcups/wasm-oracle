use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, StdResult, WasmMsg};

use crate::{
    msg::ExecuteMsg,
    state::{ADDRESSES, INFORMATION},
    ContractError,
};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn check_duplicate_addresses(addresses: Vec<String>) -> Result<(), ContractError> {
    for i in 0..addresses.len() {
        for j in i + 1..addresses.len() {
            if addresses[i] == addresses[j] {
                return Err(ContractError::DuplicateAddress {
                    address: addresses[i].clone(),
                });
            }
        }
    }
    Ok(())
}

pub fn is_address_allowed_to_send(deps: &DepsMut, sender: &str) -> Result<(), ContractError> {
    // permissioned impl. In the future we can change if the contract is permissionless
    if ADDRESSES.may_load(deps.storage, sender)?.is_none() {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

// pub fn is_data_id_allowed(deps: &DepsMut, id: &str) -> Result<(), ContractError> {
//     // permissioned impl. In the future we can change if the contract is permissionless
//     // This would not be required if we require all data to be sent for what is accepted
//     if ALLOWED_DATA.may_load(deps.storage, id)?.is_none() {
//         return Err(ContractError::InvalidId { id: id.to_string() });
//     }
//     Ok(())
// }

pub fn is_submission_within_rate_limit_rate(
    deps: &DepsMut,
    wallet: &str,
    current_height: u64,
) -> Result<(), ContractError> {
    // may want to change this to be every X blocks for all, kinda like twap. But for now, this is fine?
    // Do we really even need?

    // get last send
    let last_send = ADDRESSES.may_load(deps.storage, wallet)?.unwrap_or(0);

    if last_send == 0 {
        return Ok(());
    }

    let max_submit_rate = INFORMATION.load(deps.storage)?.max_submit_block_rate;

    let spread = current_height - last_send;

    if spread < max_submit_rate {
        return Err(ContractError::SubmittingTooQuickly {
            blocks: max_submit_rate - spread,
        });
    }
    Ok(())
}
