import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, coin, calculateFee } from "cosmwasm";

// import * as fs from 'fs';
import { getAccountFromMnemonic, getBalance } from "./helpers";

import {Averages, Data} from './types';

import {YahooFinanceProvider} from './providers/YahooFinance';
import {GoogleFinanceProvider} from './providers/GoogleFinance';

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


// async main function
async function main() {
    let data = await getAccountFromMnemonic(mnemonic, PREFIX);
    
    const balance = await getBalance(data.account.address, DENOM);    
    console.log("address: ", data.account.address);
    console.log("Balance: ", balance);


    let providers = [
        new YahooFinanceProvider(),
        new GoogleFinanceProvider(),
    ].filter(provider => provider.isEnabled());

    // Gets all data in an array (duplicate ids)
    let all_data: Data[] = [];    

    const promises = providers.map(provider => provider.getPrices());
    const results = await Promise.allSettled(promises);
    results.forEach(result => {
        if (result.status === 'fulfilled') {
            all_data = all_data.concat(result.value);
        } else {
            console.log("Error: ", result.reason);
        }
    });

    // Loop over all the data and sum it all up
    let prices_avg: Averages = {};
    for (const price of all_data) {        
        let total = price.value;
        let count = 1;
        
        if (prices_avg[price.id]) {
            total += prices_avg[price.id].total;
            count += prices_avg[price.id].count;
        }

        prices_avg[price.id] = {total, count}
    }
    // console.log("prices_avg: ", prices_avg);
        
    let data_arr: Data[] = [];
    // data_arr = [{ id: 'JUNO', value: 1580000 },{ id: 'OSMO', value: 1100000 },{ id: 'ATOM', value: 14940500 }]

    // Iterate over the averages, calulate, and push to data_arr
    for (const [k, v] of Object.entries(prices_avg)) {
        data_arr.push({
            id: k,
            value: Math.round(v.total / v.count)
        });
    }
    console.log("data_arr: ", data_arr);

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

