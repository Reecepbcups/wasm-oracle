use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub addresses: Vec<String>, // if empty = permissionless

    pub data: Vec<Identifier>,

    pub admin: Option<String>,

    // TODO: users should be able to submit as often as they want (within the limits of the average std deviation)
    pub max_submit_rate: Option<u64>,

    pub max_downtime_allowed: Option<u64>, // 86400/6 = 14400 blocks = 24 hours default

    // TWAP
    pub twap_max_blocks_length: Option<u64>, // # of blocks to store for the TWAP (average)
    pub twap_distance_between_saves: Option<u64>, // # of blocks to wait between saving the TWAP. You want this to be longer to get done more with less
}

#[cw_serde]
pub enum ExecuteMsg {
    Submit { data: Vec<Data> }, // on submit, they get a small amount of rewards from the contract

    // Admin based
    AddAddress { address: String },
    RemoveAddress { address: String },
    AddId { id: String, exponent: u8 },
    RemoveId { id: String },
    // permissionless future
    // Register {}   (requires funds to be sent as well, set via config)

    // Either slash their holdings OR a gov msg to slash validators if its a validator oracle (like x/oracle from Terra)
    // DowntimeSlash { address: String }, // requires people to setup bots. They get a small % of funds if they find someone who was down for too long
    // ^ if wallet block is 0, don't slash them yet?

    // all values are handled as value/1_000_000
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

// === Messages / Structures for State ====
#[cw_serde]
pub struct TWAPValues {
    pub blocks: Vec<u64>,
    pub values: Vec<u64>,
}

#[cw_serde]
pub struct Data {
    pub id: String,
    pub value: u64,
}

#[cw_serde]
pub struct Identifier {
    pub id: String,
    pub exponent: u8, // ex: 6 = 10**6. So on query, divide by 10**6 to get the decimal representation
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
    pub exponent: u8, // from Identifier from ALLOWED_DATA
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
pub struct TWAPValueResponse {
    pub exponent: u8, // from Identifier from ALLOWED_DATA
    pub twap_value: u64,
    pub number_of_values: u64,
}

#[cw_serde]
pub struct AllTwapValuesResponse {
    pub exponent: u8, // from Identifier from ALLOWED_DATA
    pub all_values: Vec<(u64, u64)>,
}
