use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub addresses: Vec<String>, // if empty = permissionless

    pub data: Vec<Identifier>,

    pub admin: Option<String>,
    pub max_submit_rate: Option<u64>, // 5 by default

    pub max_downtime_allowed: Option<u64>, // 86400/6 = 14400 blocks = 24 hours default
}

#[cw_serde]
pub struct Identifier {
    pub id: String,    
    pub exponent: u8, // ex: 6 = 10**6. So on query, divide by 10**6 to get the decimal representation
}

#[cw_serde]
pub enum ExecuteMsg {
    // AddAddress { address: String },
    // RemoveAddress { address: String },
    // AddDenom { denom: String },
    // RemoveDenom { denom: String },

    // all values are handled as value/1_000_000    
    Submit { id: String, value: u64 }, // on submit, they get a small amount of rewards from the contract

    // Register {}   (requires funds to be sent as well, set via config)
    // DowntimeSlash { address: String }, // requires people to setup bots. They get a small % of funds if they find someone who was down for too long
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResponse)]
    Value {
        id: String,
        measure: String, // mean/average or median
    },
    
    #[returns(AllValuesResponse)]
    AllValues { id: String },

    #[returns(AddressesResponse)]
    Addresses {},

    #[returns(WalletsValuesResponse)]
    WalletsValues { address: String },

    #[returns(ContractInformationResponse)]
    ContractInfo { },
}

// === RESPONSES ===

#[cw_serde]
pub struct ContractInformationResponse {
    pub admin: String,
    pub max_submit_block_rate: u64,
    pub max_block_downtime_allowed: u64,
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
    pub current_block: u64,
    pub last_submit_block: u64,
    pub values: Vec<(String, u64)>,
}

#[cw_serde]
pub struct AllValuesResponse {
    pub values: Vec<u64>,
}
