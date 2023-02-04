import { Data, Provider, Average, Averages } from '../types';

// https://docs.binance.us/#get-candlestick-data

// config
const REQUESTED_SYMBOLS = { "ATOM": ["ATOMUSD", "ATOMUSDT"] }

const RestHost = "https://api.binance.us"; // "https://api1.binance.com"; // non us
const RestPath = "/api/v3/ticker/price"

export class BinanceProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Binance";
    }

    async getPrices(): Promise<Data[]> {
        const res = await fetch(`${RestHost}${RestPath}`);
        const data = await res.json();        
        let data_arr: Data[] = []

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