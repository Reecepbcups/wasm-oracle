use cosmwasm_std::Deps;
use cw_storage_plus::Map;

pub const ALLOWED_DENOMS: Map<&str, bool> = Map::new("allowed_denoms");

// Address, lastBlockSubmitedFor
pub const ADDRESSES: Map<&str, u64> = Map::new("addresses");

// denom: {address1: price, address2: price}
// pub const PRICES: Map<&str, Submissions> = Map::new("prices");

// (denom, address): price
pub const PRICES: Map<(&str, &str), u64> = Map::new("prices");

// #[cw_serde]
// pub struct Submissions {
//     pub address: String,
//     pub price: u64,
// }

// get all prices for a denom by the first key in the tuple
pub fn get_prices(deps: Deps, denom: &str) -> Vec<u64> {
    let v = PRICES
        .prefix(denom)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .into_iter()
        .map(|x| x.unwrap().1)
        .collect();

    v
}

pub fn get_median_price(deps: Deps, denom: &str) -> u64 {
    let mut v = get_prices(deps, denom);
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

pub fn get_average_price(deps: Deps, denom: &str) -> u64 {
    let v = get_prices(deps, denom);
    let len = v.len();
    if len == 0 {
        return 0;
    }
    v.iter().sum::<u64>() / len as u64
}

pub fn get_wallets_submitting_price(deps: Deps, wallet: &str) -> Vec<(String, u64)> {
    // loop through PRICES and get all matches of the 2nd key in the tuple
    // let v: Vec<String> = PRICES
    //     .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    //     .into_iter()
    //     .filter(|x| x.unwrap().0 .1 == wallet)
    //     .map(|x| x.unwrap().0 .0)
    //     .collect();

    // let v: Vec<(String, u64)> = PRICES
    //     .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    //     .into_iter()
    //     .filter(|x| x.as_ref().unwrap().0 .1 == wallet)
    //     .map(|x| (x.as_ref().unwrap().0 .0, x.unwrap().1))
    //     .collect();

    // let v: Vec<(String, u64)> = PRICES
    //     .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    //     .into_iter()
    //     .filter(|x| x.as_ref().unwrap().0 .1 == wallet)
    //     .map(|x| (x.unwrap().0 .0, x.unwrap().1))
    //     .collect();

    let v: Vec<(String, u64)> = PRICES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .into_iter()
        .filter_map(|x| match x {
            Ok(((a, b), c)) if b == wallet => Some((a, c)),
            _ => None,
        })
        .collect();

    v
}
