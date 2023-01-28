#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AddressesResponse, ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};

use crate::state::{Submissions, ADDRESSES, PRICES};

// const CONTRACT_NAME: &str = "crates.io:oracle";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Ensure all addresses are good
    msg.addresses.iter().for_each(|address| {
        deps.api.addr_validate(address).unwrap();
    });

    msg.addresses.iter().for_each(|address| {
        ADDRESSES
            .save(deps.storage, address.as_str(), &env.block.height)
            .unwrap();
    });

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SubmitPrice { denom, price } => {
            // permissioned address send check
            if ADDRESSES
                .may_load(deps.storage, _info.sender.as_str())?
                .is_none()
            {
                return Err(ContractError::Unauthorized {});
            }

            let submit = Submissions {
                address: _info.sender.to_string(),
                price,
            };

            PRICES.save(deps.storage, denom.as_str(), &submit)?;

            Ok(Response::new().add_attribute("action", "submit_price"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { denom } => {
            let price = PRICES.may_load(deps.storage, denom.as_str())?;

            match price {
                Some(price) => Ok(to_binary(&PriceResponse {
                    denom,
                    price: price.price,
                })?),
                None => Ok(to_binary(&PriceResponse { denom, price: 0 })?),
            }
        }

        QueryMsg::Addresses {} => {
            let addresses: Result<_, _> = ADDRESSES
                .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .into_iter()
                .collect();

            match addresses {
                Ok(addresses) => Ok(to_binary(&AddressesResponse { addresses })?),
                Err(_) => Ok(to_binary(&AddressesResponse { addresses: vec![] })?),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
