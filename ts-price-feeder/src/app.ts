import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, coin, calculateFee } from "cosmwasm";

// import * as fs from 'fs';
import { getAccountFromMnemonic, getBalance } from "./helpers";

import { CoinGeckoClient } from 'coingecko-api-v3';

const DENOM = "ujunox";
const PREFIX = "juno";
const GAS = GasPrice.fromString(`0.003${DENOM}`);

const CONTRACT_ADDRESS = "juno1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq68ev2p";

export const rpcEndpoint = "http://localhost:26657";

const config = {
    chainId: "testing",
    rpcEndpoint: rpcEndpoint,
    prefix: "juno",
    gasPrice: GAS,
};

const fee = calculateFee(200_000, config.gasPrice);

// juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
const mnemonic = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";


async function submit_tx(client: SigningCosmWasmClient, account_addr, execute_msg) {    
    let res = await client.execute(account_addr, CONTRACT_ADDRESS, execute_msg, fee, "memo").catch((err) => {
        console.log("Error: ", err);
        return undefined;
    }).then((res) => {        
        return res;      
    });

    if (res) {
        console.log("tx_hash: ", res.transactionHash);
        console.log("height: ", res.height);
        console.log("logs: ", res.logs[0].log);
    }
}

interface Data {
    id: string;
    value: number;
}

// coingecko to denom loopup map
const COINGECKO_DENOM_MAP = {
    "juno-network": "JUNO",
    "osmosis": "OSMO"
}

// async main function
async function main() {
    let data = await getAccountFromMnemonic(mnemonic, PREFIX);
    
    const balance = await getBalance(data.account.address, DENOM);    
    console.log("address: ", data.account.address);
    console.log("Balance: ", balance);

    const coingecko = new CoinGeckoClient({
        timeout: 10000,
        autoRetry: true,
    });
    const prices = await coingecko.simplePrice({vs_currencies: 'usd', ids: 'juno-network,osmosis'});
    // console.log(prices);
    // let price = Number(prices['juno-network'].usd) * 10**6;

    
    // [ { id: 'JUNO', value: 1480000 }, { id: 'OSMO', value: 972506 } ]
    let data_arr: Data[] = [] 

    for (const key of Object.keys(prices)) {
        let price = Number(prices[key].usd) * 10**6;
        data_arr.push({
            id: COINGECKO_DENOM_MAP[key],
            value: price
        });
    }

    console.log(data_arr);

    const client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, data.wallet, config);

    // {"submit":{"id":"JUNO","value":1000000}}
    let execute_msg = {
        submit: { data: data_arr }
    }

    await submit_tx(client, data.account.address, execute_msg);
    

    let query = await client.queryContractSmart(CONTRACT_ADDRESS, {
        wallets_values: {
            address: data.account.address
        }
    });
    console.log("wallets_values query: ", query);
}

main()

