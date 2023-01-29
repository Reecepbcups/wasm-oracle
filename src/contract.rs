#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AddressesResponse, AllTwapValuesResponse, AllValuesResponse, ContractInformationResponse,
    ExecuteMsg, InstantiateMsg, QueryMsg, TWAPValueResponse, ValueResponse, WalletsValuesResponse,
};

use crate::state::{
    get_average_value, get_last_submit_block, get_median_value, get_twap,
    get_twap_blocks_and_values, get_twap_if_it_is_time, get_values, get_wallets_submitting_values,
    ADDRESSES, ALLOWED_DATA, INFORMATION, VALUES,
};

use crate::helpers::{
    check_duplicate_addresses, is_address_allowed_to_send, is_submission_within_rate_limit_rate,
};

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

    let twap_max_blocks_length = msg.twap_max_blocks_length.unwrap_or(100);
    let twap_distance_between_saves = msg.twap_distance_between_saves.unwrap_or(100);

    INFORMATION.save(
        deps.storage,
        &ContractInformationResponse {
            admin,
            max_submit_block_rate: max_submit_rate,
            max_block_downtime_allowed,

            twap_max_blocks_length,
            twap_distance_between_saves,
            twap_last_save_block_actual: 0,
        },
    )?;

    // save allowed data ids to state
    msg.data.iter().for_each(|data| {
        ALLOWED_DATA
            .save(deps.storage, data.id.as_str(), &data.exponent)
            .unwrap();
    });

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {

        // TODO: this is so gas computational. Is it even feasible to have TWAP? Maybe we compute TWAP on query?
        ExecuteMsg::Submit { data } => {
            // is_data_id_allowed(&deps, id.as_str())?;
            is_address_allowed_to_send(&deps, info.sender.as_str())?;
            is_submission_within_rate_limit_rate(&deps, info.sender.as_str(), env.block.height)?; // remove ?

            // check other values, if too far off (+/- X%), SLASH THEM / remove from list (make configurable). THen do not put value in.
            // value_difference()

            // require all ids to be submitted on?

            // VALUES.save(deps.storage, (id.as_str(), info.sender.as_str()), &value)?;

            // VALUES.update(deps.storage, (info.sender.as_str(), data.id.as_str()), |old| -> StdResult<_> {
            //     Ok(data.value)
            // })?;

            // iterate over data and save each to deps.storage without moving deps.storage
            data.iter().for_each(|data| {
                VALUES
                    .save(
                        deps.storage,
                        (info.sender.as_str(), data.id.as_str()),
                        &data.value,
                    )
                    .unwrap();
            });

            ADDRESSES.update(deps.storage, info.sender.as_str(), |_| -> StdResult<_> {
                Ok(env.block.height)
            })?;

            let mut info = INFORMATION.load(deps.storage)?;

            data.iter().for_each(|data| {
                if let Ok(twap) =
                    get_twap_if_it_is_time(deps.as_ref(), &info, &data.id, env.block.height)
                {                    
                    if twap.is_none() {
                        return;
                    }

                    // update twap if it is time. The average of all is saved here
                    crate::state::TWAP
                        .save(deps.storage, data.id.as_str(), &twap.unwrap().0)
                        .unwrap();

                    // update info
                    info.twap_last_save_block_actual = env.block.height;
                    INFORMATION.save(deps.storage, &info).unwrap();
                }
            });

            Ok(Response::new().add_attribute("action", "submit_data"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value { id, measure } => {
            let value: u64 = match measure.as_ref() {
                "median" => get_median_value(deps, id.as_str()),
                _ => get_average_value(deps, id.as_str()),
            };

            // let info = INFORMATION.load(deps.storage)?;

            let exponent = ALLOWED_DATA.load(deps.storage, id.as_str())?;

            return to_binary(&ValueResponse {
                id: id.as_str(),
                value,
                exponent,
            });
        }

        QueryMsg::AllValues { id } => {
            let values = get_values(deps, id.as_str());
            let all_values_response = AllValuesResponse { values };
            to_binary(&all_values_response)
        }

        QueryMsg::TwapValue { id } => {
            let value_avg = get_twap(deps, id.as_str());
            let exponent = ALLOWED_DATA.load(deps.storage, id.as_str())?;

            to_binary(&TWAPValueResponse {
                exponent,
                twap_value: value_avg.0,
                number_of_values: value_avg.1,
            })
        }

        QueryMsg::AllTwapValues { id } => {
            let all_values = get_twap_blocks_and_values(deps, id.as_str());
            let exponent = ALLOWED_DATA.load(deps.storage, id.as_str())?;

            to_binary(&AllTwapValuesResponse {
                exponent,
                all_values,
            })
        }

        QueryMsg::WalletsValues { address } => {
            let v = get_wallets_submitting_values(deps, address.as_str());

            let current_block = env.block.height;
            let last_submit_block = get_last_submit_block(deps, address.as_str());

            Ok(to_binary(&WalletsValuesResponse {
                last_submit_block,
                current_block,
                values: v,
            })?)
        }

        QueryMsg::ContractInfo {} => {
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

            to_binary(&addresses_response)
        }
    }
}

#[cfg(test)]
mod tests {}
