#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AddressesResponse, AllDenomPrices, ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg,
    WalletsPricesResponse,
};

use crate::state::{
    get_average_price, get_median_price, get_prices, get_wallets_submitting_price, ADDRESSES,
    ALLOWED_DENOMS, PRICES,
};

use crate::helpers::check_duplicate_addresses;

// const CONTRACT_NAME: &str = "crates.io:oracle";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    msg.addresses.iter().for_each(|address| {
        deps.api.addr_validate(address).unwrap();
    });

    check_duplicate_addresses(msg.addresses.clone())?;

    msg.addresses.iter().for_each(|address| {
        ADDRESSES
            .save(deps.storage, address.as_str(), &env.block.height)
            .unwrap();
    });

    // save msg.denoms to state
    msg.denoms.iter().for_each(|denom| {
        ALLOWED_DENOMS
            .save(deps.storage, denom.as_str(), &true)
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
        ExecuteMsg::SubmitPrice { denom, price } => {
            // Ensure denom is allowed
            let value = ALLOWED_DENOMS.may_load(deps.storage, &denom)?;
            if value.is_none() || !value.unwrap() {
                return Err(ContractError::InvalidDenom { denom });
            }

            // Ensure they are allowed to send in prices (permissioned)
            // TODO: allow permissionless sends in the future?
            if ADDRESSES
                .may_load(deps.storage, info.sender.as_str())?
                .is_none()
            {
                return Err(ContractError::Unauthorized {});
            }

            // TODO: only allow send every X blocks? (init msg: 5 default)
            // check other prices, if too far off, SLASH THEM / remove from list (make configurable). THen do not put price in.

            PRICES.save(deps.storage, (denom.as_str(), info.sender.as_str()), &price)?;

            // ADDRESSES.save(deps.storage, info.sender.as_str(), &env.block.height)?;
            ADDRESSES.update(deps.storage, info.sender.as_str(), |_| -> StdResult<_> {
                Ok(env.block.height)
            })?;

            Ok(Response::new().add_attribute("action", "submit_price"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllDenomPrices { denom } => {
            let prices = get_prices(deps, denom.as_str());
            Ok(to_binary(&AllDenomPrices { prices })?)
        }

        QueryMsg::Price { denom, measure } => {
            // let price = PRICES.may_load(deps.storage, denom.as_str())?;
            // match price {
            //     Some(price) => Ok(to_binary(&PriceResponse {
            //         denom,
            //         price: price.price,
            //     })?),
            //     None => Ok(to_binary(&PriceResponse { denom, price: 0 })?),
            // }

            // get_prices(deps, denom.as_str())
            //     .into_iter()
            //     .max()
            //     .map(|price| {
            //         to_binary(&PriceResponse {
            //             denom: denom.as_str(),
            //             price,
            //         })
            //     })
            //     .unwrap_or(Ok(to_binary(&PriceResponse {
            //         denom: denom.as_str(),
            //         price: 0,
            //     })?))
            
            match measure.as_ref() {
                "median" => {
                    let value: u64 = get_median_price(deps, denom.as_str());

                    return to_binary(&PriceResponse {
                        denom: denom.as_str(),
                        price: value,
                    });
                }
                _ => {
                    let value: u64 = get_average_price(deps, denom.as_str());
                    Ok(to_binary(&PriceResponse {
                        denom: denom.as_str(),
                        price: value,
                    })?)
                }
            }
        }

        QueryMsg::WalletsPrices { address } => {
            let v = get_wallets_submitting_price(deps, address.as_str());
            Ok(to_binary(&WalletsPricesResponse { prices: v })?)
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
