use coingecko::CoinGeckoClient;

use cosmrs::{
    // bank::MsgSend,       
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo, Tx},
    AccountId, Coin,
    bip32,    
};

// use cosmrs::tendermint::PrivateKey;

use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::rpc;

// use serde::{Serialize};
use serde_json;

use oracle::msg::ExecuteMsg::Submit;

// pub use cosmos_sdk_proto as proto;
// pub use tendermint;
// pub use bip32;
// pub use tendermint_rpc as rpc;


const DENOM: &str = "ujuno";
const ACCOUNT_NUMBER: AccountNumber = 1; // ? 3?

const MEMO: &str = "test memo"; 

const RPC_ADDRESS: &str = "http://localhost:26657";
const PREFIX: &str = "juno";
const CONTRACT: &str = "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8";

#[tokio::main]
async fn main() {

    let chain_id = "juno-1".parse().unwrap();

    let mnemonic = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";
    let account = bip32::Mnemonic::new(mnemonic, bip32::Language::English).unwrap();

    let seed = account.to_seed("");
    let acc_bytes = seed.as_bytes();

    
    // print acc_bytes
    println!("acc_bytes: {:?}", acc_bytes);

    // panic!(" ");    

    let amount = Coin {
        amount: 5000u128,
        denom: DENOM.parse().unwrap(),
    };
    
    let sequence_number = 0;
    let gas = 500_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    // let sender_private_key = secp256k1::SigningKey::random();
    let sender_private_key = secp256k1::SigningKey::from_bytes(acc_bytes).unwrap(); // this work?
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(PREFIX).unwrap();
    
    println!("sender_public_key: {:?}", sender_public_key);    
    println!("sender_account_id: {:?}", sender_account_id);
        

    // convert CONTRACT to AccountId 
    let contract_account_id = AccountId::new(PREFIX, CONTRACT.as_bytes()).unwrap();
    
    println!("contract_account_id: {:?}", contract_account_id);


    // let client = CoinGeckoClient::default();    
    // let v = client.price(&["juno-network", "osmosis"], &["usd"], true, true, true, true).await;
    // println!("{:?}", v);

    let s = Submit {
        id: "JUNO".to_string(),
        value: 10_000_000,
    };

    // seralize s as json
    let json = serde_json::to_string(&s).unwrap();

    // convert json to bytes as a Vec<u8>
    let bytes = json.as_bytes().to_vec();

    let msg = MsgExecuteContract {
        sender: sender_account_id,
        contract: contract_account_id,
        msg: bytes,
        funds: vec![],
    }.to_any().unwrap();    

    let tx_body = tx::BodyBuilder::new().msg(msg).memo(MEMO).finish();
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();


    let rpc_client = rpc::HttpClient::new(RPC_ADDRESS).unwrap();
    let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();
    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    }

    if tx_commit_response.deliver_tx.code.is_err() {
        panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
    }

    println!("tx_commit_response: {:?}", tx_commit_response);
}
