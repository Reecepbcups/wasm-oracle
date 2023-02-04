import { Data, Provider } from '../types';

import { default as config } from '../../config.json';

import * as googleFinance from 'google-finance';

const CONFIG = config.google_finance;
const TICKERS = CONFIG.tickers;

export class GoogleFinanceProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Google Finance";
    }

    isEnabled(): boolean {
        return CONFIG.enabled;
    }

    async getPrices(): Promise<Data[]> {
        let data_arr: Data[] = [];

        let promises = [];
        for (const symbol of TICKERS) {
            promises.push([symbol, this.getStockPrice(symbol)]);
        }

        // all Settled the 1st index in the array
        const results = await Promise.allSettled(promises.map(p => p[1]));
        for (const result of results) {
            if (result.status === 'fulfilled') {
                data_arr.push({
                    id: promises[results.indexOf(result)][0], // symbol
                    value: result.value
                });
            }
        }

        return data_arr;
    }

    async getStockPrice(symbol: string): Promise<number> {
        let v = await googleFinance.quote({
            symbol: symbol.toUpperCase(),
            modules: ['price'] // see the docs for the full list  'summaryDetail'
        }, function (err, quotes) {
            // ...
        });

        if (v.price.regularMarketPrice) {
            return v.price.regularMarketPrice * 10 ** 6;
        } else {
            return -1;
        }
    }
}
