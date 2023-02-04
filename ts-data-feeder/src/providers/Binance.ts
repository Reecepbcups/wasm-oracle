import { Data, Provider } from '../types';

import {default as config} from '../../config.json';

// https://docs.binance.us/#get-candlestick-data

const CONFIG = config.binance;

// "https://api1.binance.com"; // non us
const RestHost = CONFIG.rest_host; 
const RestPath = CONFIG.rest_path;
const REQUESTED_SYMBOLS = CONFIG.symbols;

console.log(REQUESTED_SYMBOLS, Object.keys(REQUESTED_SYMBOLS))

export class BinanceProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Binance";
    }

    isEnabled(): boolean {
        return CONFIG.enabled;
    }

    async getPrices(): Promise<Data[]> {
        let data_arr: Data[] = []        

        const res = await fetch(`${RestHost}${RestPath}`);     
        const data = await res.json()
        
        if ('msg' in data && data.msg.startsWith('Service unavailable from a restricted location')) {
            console.log("ERROR: Binance API call failed. Check your config.json for binance to use the non-us endpoint `https://api.binance.com`")
            return data_arr;
        }

        // [{ id: 'ATOMUSDT', value: 14974000 },{ id: 'ATOMUSD', value: 14956000 }]
        const d = data
            .filter(d => Object.keys(REQUESTED_SYMBOLS).some(k => d.symbol.startsWith(k) && REQUESTED_SYMBOLS[k].includes(d.symbol)))
            .map(d => ({
                id: d.symbol,
                value: Number(d.price) * 10 ** 6
            }));                

                
        // ATOMUSDT becomes ATOM. This means there ARE duplicates in the array.
        // we sort those in the main function after calling all providers
        for(const price of d) {
            for (const [k, v] of Object.entries(REQUESTED_SYMBOLS)) {
                if (v.includes(price.id)) {
                    data_arr.push({
                        id: k,
                        value: price.value
                    });
                }
            }
        }

        return data_arr;
    }
}