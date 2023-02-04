import { Data, Provider, Average, Averages } from '../types';

// config
const REQUESTED_SYMBOLS = { "ATOM": ["ATOM-USD", "ATOM-USDT"] }

// https://api.exchange.coinbase.com/products/ATOM-USDT/ticker
const RestHost = "https://api.exchange.coinbase.com";
const RestPath = "/products/%PAIR%/ticker"

export class CoinbaseProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Coinbase";
    }

    async getPrices(): Promise<Data[]> {            
        let data_arr: Data[] = [];

        for(const [k, v] of Object.entries(REQUESTED_SYMBOLS)) {

            // loop through v (probably need to do this as a Promise.allSettled)
            for(const ticker of v) {
                // https://api.exchange.coinbase.com/products/ATOM-USDT/ticker
                const res = await fetch(`${RestHost}${RestPath.replace("%PAIR%", ticker)}`);
                const data = await res.json();
                // console.log(data);
                data_arr.push({
                    id: k,
                    value: Number(data.price) * 10 ** 6
                });
            }     
                   
        }      

        return data_arr;
    }
}