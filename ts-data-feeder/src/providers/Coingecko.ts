// NOTE: this is purely an example, you should NOT use coingecko in a production setting...

import { CoinGeckoClient } from 'coingecko-api-v3';

import {Data, Provider} from '../types';

// coingecko to denom lookup map (TODO: This would need to be in the config file)
const COINGECKO_DENOM_MAP = {
    "juno-network": "JUNO",
    "osmosis": "OSMO"
}

export class CoinGeckoProvider implements Provider {
    name: string;
    coingecko: CoinGeckoClient;

    constructor() {
        this.name = "CoinGecko";
        this.coingecko = new CoinGeckoClient({
            timeout: 10000,
            autoRetry: true,
        });
    }

    async getPrices(): Promise<Data[]> {
        // throw new Error("Method not implemented.");
        const v = await this.coingecko.simplePrice({vs_currencies: 'usd', ids: 'juno-network,osmosis'});
        // return v;
        let data_arr: Data[] = []
        for (const key of Object.keys(v)) {
            let price = Number(v[key].usd) * 10**6;

            // if key not in COINGECKO_DENOM_MAP, then use key as id
            let id = key;
            if (key in COINGECKO_DENOM_MAP) {
                id = COINGECKO_DENOM_MAP[key];
            }

            data_arr.push({
                id: id,                
                value: price
            });
        }        

        return data_arr;
    }
}