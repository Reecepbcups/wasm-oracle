// NOTE: this is purely an example, you should NOT use coingecko in a production setting...

import { CoinGeckoClient } from 'coingecko-api-v3';

import { Data, Provider } from '../types';

// config file
const REQUESTED_SYMBOLS = {
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
        const ids = Object.keys(REQUESTED_SYMBOLS).join(',');

        const v = await this.coingecko.simplePrice({ vs_currencies: 'usd', ids });

        let data_arr: Data[] = []
        for (const key of Object.keys(v)) {
            let value = Number(v[key].usd) * 10 ** 6;

            // if key not in COINGECKO_DENOM_MAP, then use key as id
            let id = key;
            if (key in REQUESTED_SYMBOLS) {
                id = REQUESTED_SYMBOLS[key];
            }                    

            data_arr.push({
                id,
                value
            });
        }

        return data_arr;
    }
}