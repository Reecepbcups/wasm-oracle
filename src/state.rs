use cosmwasm_std::Deps;
use cw_storage_plus::{Item, Map};

use crate::msg::ContractInformationResponse;

pub const INFORMATION: Item<ContractInformationResponse> = Item::new("info");

pub const ALLOWED_DATA: Map<&str, bool> = Map::new("allowed");

// Address, lastBlockSubmitedFor
pub const ADDRESSES: Map<&str, u64> = Map::new("addresses");

// Ex: (denom, address): price
pub const VALUES: Map<(&str, &str), u64> = Map::new("values");

// get all values for a id by the first key in the tuple
pub fn get_values(deps: Deps, denom: &str) -> Vec<u64> {
    let v = VALUES
        .prefix(denom)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .into_iter()
        .map(|x| x.unwrap().1)
        .collect();

    v
}

pub fn get_median_value(deps: Deps, denom: &str) -> u64 {
    let mut v = get_values(deps, denom);
    v.sort();
    let len = v.len();
    if len == 0 {
        return 0;
    }
    if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) / 2
    } else {
        v[len / 2]
    }
}

pub fn get_average_value(deps: Deps, denom: &str) -> u64 {
    let v = get_values(deps, denom);
    let len = v.len();
    if len == 0 {
        return 0;
    }
    v.iter().sum::<u64>() / len as u64
}

pub fn get_wallets_submitting_values(deps: Deps, wallet: &str) -> Vec<(String, u64)> {
    let v: Vec<(String, u64)> = VALUES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .into_iter()
        .filter_map(|x| match x {
            Ok(((a, b), c)) if b == wallet => Some((a, c)),
            _ => None,
        })
        .collect();

    v
}
