#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AddressesResponse, AllValuesResponse, ContractInformationResponse, ExecuteMsg, InstantiateMsg,
    QueryMsg, ValueResponse, WalletsValuesResponse,
};

use crate::state::{
    get_average_value, get_median_value, get_values, get_wallets_submitting_values, ADDRESSES,
    ALLOWED_DATA, INFORMATION, VALUES, get_last_submit_block,
};

use crate::helpers::check_duplicate_addresses;

// const CONTRACT_NAME: &str = "crates.io:oracle";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    check_duplicate_addresses(msg.addresses.clone())?;
    
    // if addresses.length == 0, its permissionless
    msg.addresses.iter().for_each(|address| {
        deps.api.addr_validate(address).unwrap();

        ADDRESSES
            .save(deps.storage, address.as_str(), &env.block.height)
            .unwrap();
    });

    
    // see if msg.admin is set, if not, use info.sender
    let admin = msg.admin.unwrap_or_else(|| info.sender.into_string());    
    deps.api.addr_validate(&admin)?;
    
    let max_submit_rate = msg.max_submit_rate.unwrap_or(5);
    let max_block_downtime_allowed = msg.max_downtime_allowed.unwrap_or(14400); // 24 hours @ 6 seconds = 14400 blocks

    INFORMATION.save(
        deps.storage,
        &ContractInformationResponse {
            admin,
            max_submit_block_rate: max_submit_rate,
            max_block_downtime_allowed
        },
    )?;

    // save allowed data ids to state
    msg.data.iter().for_each(|data| {
        ALLOWED_DATA
            .save(deps.storage, data.id.as_str(), &true)
            .unwrap();
    });

    Ok(Response::new().add_attribute("action", "instantiate"))
}

fn is_address_allowed_to_send(deps: &DepsMut, sender: &str) -> Result<(), ContractError> {
    // permissioned impl. In the future we can change if the contract is permissionless
    if ADDRESSES.may_load(deps.storage, sender)?.is_none() {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

fn is_data_id_allowed(deps: &DepsMut, denom: &str) -> Result<(), ContractError> {
    // permissioned impl. In the future we can change if the contract is permissionless
    if ALLOWED_DATA.may_load(deps.storage, denom)?.is_none() {
        return Err(ContractError::InvalidDenom {
            denom: denom.to_string(),
        });
    }
    Ok(())
}

fn is_submission_within_rate_limit_rate(
    deps: &DepsMut,
    wallet: &str,
    current_height: u64,
) -> Result<(), ContractError> {
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Submit { id, value } => {
            is_data_id_allowed(&deps, id.as_str())?;
            is_address_allowed_to_send(&deps, info.sender.as_str())?;

            // Only allow send every X blocks (init msg: 5 default)
            is_submission_within_rate_limit_rate(&deps, info.sender.as_str(), env.block.height)?;

            // check other values, if too far off, SLASH THEM / remove from list (make configurable). THen do not put value in.
            // value_difference()

            // require all ids to be submited on.

            VALUES.save(deps.storage, (id.as_str(), info.sender.as_str()), &value)?;

            ADDRESSES.update(deps.storage, info.sender.as_str(), |_| -> StdResult<_> {
                Ok(env.block.height)
            })?;

            Ok(Response::new().add_attribute("action", "submit_data"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllValues { id } => {
            let values = get_values(deps, id.as_str());            

            let all_values_response = AllValuesResponse { values };
            return to_binary(&all_values_response);
        }

        QueryMsg::Value { id, measure } => {
            let value: u64 = match measure.as_ref() {
                "median" => get_median_value(deps, id.as_str()),
                _ => get_average_value(deps, id.as_str()),
            };

            return to_binary(&ValueResponse {
                id: id.as_str(),
                value,
            });
        }

        QueryMsg::WalletsValues { address } => {
            let v = get_wallets_submitting_values(deps, address.as_str());

            let current_block = env.block.height;
            let last_submit_block = get_last_submit_block(deps, address.as_str());


            Ok(to_binary(&WalletsValuesResponse { last_submit_block, current_block, values: v })?)
        }

        QueryMsg::ContractInfo {  } => {
            let info = INFORMATION.load(deps.storage)?;
            let v = to_binary(&info)?;
            Ok(v)
        }

        QueryMsg::Addresses {} => {
            let addresses: Result<_, _> = ADDRESSES
                .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .into_iter()
                .collect();

            let addresses_response = match addresses {
                Ok(addresses) => AddressesResponse { addresses },
                Err(_) => AddressesResponse { addresses: vec![] },
            };

            return to_binary(&addresses_response);
        }
    }
}

#[cfg(test)]
mod tests {}
