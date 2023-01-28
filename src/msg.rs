use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub addresses: Vec<String>,
    pub denoms: Vec<String>, // coingecko ids for now, can add more providers in the future
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
    SubmitPrice { denom: String, price: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceResponse)]
    Price {
        denom: String,
        measure: String, // mean/average or median
    },

    // TODO: add ADDRESSES and their last block submits
    #[returns(AllDenomPrices)]
    AllDenomPrices { denom: String },

    #[returns(AddressesResponse)]
    Addresses {},

    #[returns(WalletsPricesResponse)]
    WalletsPrices { address: String },
}

#[cw_serde]
pub struct PriceResponse<'a> {
    pub denom: &'a str,
    pub price: u64,
}
#[cw_serde]
pub struct AddressesResponse {
    pub addresses: Vec<String>,
}

#[cw_serde]
pub struct WalletsPricesResponse {
    pub prices: Vec<(String, u64)>,
}

#[cw_serde]
pub struct AllDenomPrices {
    pub prices: Vec<u64>,
}
