use cosmwasm_schema::{cw_serde, QueryResponses};

// TODO: change anything that says "denom" and make more generalized (ex: "identifier") so other non financial data can be stored on chain

#[cw_serde]
pub struct InstantiateMsg {
    pub addresses: Vec<String>, // if empty = permissionless

    pub data: Vec<Identifier>,

    pub admin: Option<String>,
    pub max_submit_rate: Option<u64>, // 5 by default
}

#[cw_serde]
pub struct Identifier {
    pub id: String,    
    pub exponent: u8, // ex: 6 = 10**6
}

// Future: SudoMsg to slash validators?
// Save address'es last submit block height, and have a public CheckSlash function. If someone calls it and there is a slash, pay said user some JUNO as reward

#[cw_serde]
pub enum ExecuteMsg {
    // AddAddress { address: String },
    // RemoveAddress { address: String },
    // AddDenom { denom: String },
    // RemoveDenom { denom: String },

    // all values are handled as value/1_000_000
    Submit { id: String, value: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResponse)]
    Value {
        id: String,
        measure: String, // mean/average or median
    },

    // TODO: add ADDRESSES and their last block submits
    #[returns(AllValuesResponse)]
    AllValues { id: String },

    #[returns(AddressesResponse)]
    Addresses {},

    #[returns(WalletsValuesResponse)]
    WalletsValues { address: String },
}

// === RESPONSES ===

#[cw_serde]
pub struct ContractInformationResponse {
    pub admin: String,
    pub max_submit_block_rate: u64,
}

#[cw_serde]
pub struct ValueResponse<'a> {
    pub id: &'a str,
    pub value: u64,
}
#[cw_serde]
pub struct AddressesResponse {
    pub addresses: Vec<String>,
}

#[cw_serde]
pub struct WalletsValuesResponse {
    pub values: Vec<(String, u64)>,
}

#[cw_serde]
pub struct AllValuesResponse {
    pub values: Vec<u64>,
}
