use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub addresses: Vec<String>, // if empty = permissionless

    pub data: Vec<Identifier>,

    pub admin: Option<String>,
    pub max_submit_rate: Option<u64>, // 5 by default

    pub max_downtime_allowed: Option<u64>, // 86400/6 = 14400 blocks = 24 hours default

    // TWAP
    pub twap_max_blocks_length: Option<u64>, // # of blocks to store for the TWAP (average)
    pub twap_distance_between_saves: Option<u64>, // # of blocks to wait between saving the TWAP. Default 50
                                                  // 250 * 20 = 5000 blocks * 6 seconds = 5 hours
}

#[cw_serde]
pub struct Identifier {
    pub id: String,
    pub exponent: u8, // ex: 6 = 10**6. So on query, divide by 10**6 to get the decimal representation
}

#[cw_serde]
pub enum ExecuteMsg {
    // Admin based
    // AddAddress { address: String },
    // RemoveAddress { address: String },
    // AddDenom { denom: String },
    // RemoveDenom { denom: String },

    // permissionless future
    // Register {}   (requires funds to be sent as well, set via config)

    // Either slash their holdings OR a gov msg to slash validators if its a validator oracle (like x/oracle from Terra)
    // DowntimeSlash { address: String }, // requires people to setup bots. They get a small % of funds if they find someone who was down for too long

    // all values are handled as value/1_000_000
    Submit { id: String, value: u64 }, // on submit, they get a small amount of rewards from the contract
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

    #[returns(TWAPValueResponse)]
    TwapValue { id: String },

    #[returns(AllTwapValuesResponse)]
    AllTwapValues { id: String },

    #[returns(AddressesResponse)]
    Addresses {},

    #[returns(WalletsValuesResponse)]
    WalletsValues { address: String },

    #[returns(ContractInformationResponse)]
    ContractInfo {},
}

// === RESPONSES ===

#[cw_serde]
pub struct ContractInformationResponse {
    pub admin: String,

    pub max_submit_block_rate: u64,
    pub max_block_downtime_allowed: u64,

    pub twap_max_blocks_length: u64,
    pub twap_distance_between_saves: u64,

    // the last block we saved to the twap
    pub twap_last_save_block_actual: u64,
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

#[cw_serde]
pub struct TWAPValues {
    pub blocks: Vec<u64>,
    pub values: Vec<u64>,
}

#[cw_serde]
pub struct TWAPValueResponse {
    pub twap_value: u64,
}

#[cw_serde]
pub struct AllTwapValuesResponse {
    pub all_values: Vec<(u64, u64)>,
}
