use cosmwasm_std::{Deps, DepsMut};
use cw_storage_plus::{Item, Map};

use crate::{
    msg::{ContractInformationResponse, TWAPValues},
    ContractError,
};

pub const INFORMATION: Item<ContractInformationResponse> = Item::new("info");

// id: exponent
pub const ALLOWED_DATA: Map<&str, u8> = Map::new("allowed");

// Address, lastBlockSubmittedFor
pub const ADDRESSES: Map<&str, u64> = Map::new("addresses");

// Ex: (denom, address): price
pub const VALUES: Map<(&str, &str), u64> = Map::new("values");

// TWAP = on submit, every X blocks save the value here. We don't need to save every block.
// Then rotate it out like how I did my marketplace.
// ID: vec<(block, value)> max of XXX long, set every XXXX blocks
// Ex: every 100 blocks, save the average price here. Keep the last 100 instances of this data = .
// Have a function return the TWAP average
pub const TWAP: Map<&str, TWAPValues> = Map::new("twap");

pub fn get_twap_blocks_and_values(deps: Deps, id: &str) -> Vec<(u64, u64)> {
    // Vec<(u64, u64)>
    let empty_twap = TWAPValues {
        blocks: vec![],
        values: vec![],
    };
    let twap = TWAP
        .load(deps.storage, id)
        .unwrap_or_else(|_| empty_twap.clone());
    let result = twap
        .blocks
        .iter()
        .zip(twap.values.iter())
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(u64, u64)>>();
    result

    // -> TWAPValues
    // let empty_twap = TWAPValues {
    //     blocks: vec![],
    //     values: vec![],
    // };
    // let twap: TWAPValues = TWAP.load(deps.storage, id).unwrap_or_else(|_| empty_twap.clone());
    // twap
    //
    // Example of how it works:
    // for (block, value) in twap.blocks.iter().zip(twap.values.iter()) {
    //     // do something with block and value
    // }
}

pub fn get_twap(deps: Deps, id: &str) -> (u64, u64) {
    let twap = get_twap_blocks_and_values(deps, id);
    if twap.is_empty() {
        return (0, 0);
    }

    let mut sum = 0;
    for (_, value) in &twap {
        sum += value;
    }

    // (value, number of values used to calulate)
    (sum / twap.len() as u64, twap.len() as u64)
}

// If its time for a new TWAP value (saves average over time), we calculate average and save it
pub fn update_twap_if_it_is_time(deps: DepsMut, id: &str, block: u64) -> Result<(), ContractError> {
    let mut info = INFORMATION.load(deps.storage)?;

    let distance_between_saves = info.twap_distance_between_saves; // ex: 50
    let last_save_block = info.twap_last_save_block_actual; // ex: block 5,650,200

    if block < last_save_block + distance_between_saves {
        return Ok(());
    }

    let current_average = get_average_value(deps.as_ref(), id);

    let mut twap = get_twap_blocks_and_values(deps.as_ref(), id);
    twap.push((block, current_average));

    // if twap.len() > max_blocks_length, remove the first element
    if twap.len() > info.twap_max_blocks_length.try_into().unwrap() {
        twap.remove(0);
    }

    // save the new twap
    let twap = TWAPValues {
        blocks: twap.iter().map(|x| x.0).collect(),
        values: twap.iter().map(|x| x.1).collect(),
    };

    TWAP.save(deps.storage, id, &twap)?;

    // update last_save_block time since we updated
    info.twap_last_save_block_actual = block;
    INFORMATION.save(deps.storage, &info)?;

    Ok(())
}

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

pub fn get_last_submit_block(deps: Deps, address: &str) -> u64 {
    let last_submit_block = ADDRESSES.may_load(deps.storage, address);
    match last_submit_block {
        Ok(Some(last_submit_block)) => last_submit_block,
        _ => 0,
    }
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
