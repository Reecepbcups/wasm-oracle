import { Data, Provider } from '../types';

import {default as config} from '../../config.json';

const CONFIG = config.coinbase;

// https://api.exchange.coinbase.com/products/ATOM-USDT/ticker
const RestHost = CONFIG.rest_host;
const RestPath = CONFIG.rest_path;
const REQUESTED_SYMBOLS = CONFIG.symbols;

export class CoinbaseProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Coinbase";
    }

    isEnabled(): boolean {
        return CONFIG.enabled;
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