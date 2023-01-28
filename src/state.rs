use cosmwasm_schema::cw_serde;
// use schemars::Set;
use cw_storage_plus::Map;

// Address, lastBlockSubmitedFor
pub const ADDRESSES: Map<&str, u64> = Map::new("addresses");

// denom: {address1: price, address2: price}
pub const PRICES: Map<&str, Submissions> = Map::new("prices");

#[cw_serde]
pub struct Submissions {
    pub address: String,
    pub price: u64,
}
