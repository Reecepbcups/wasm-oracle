import { Data, Provider, Average, Averages } from '../types';

import {default as config} from '../../config.json';
const CONFIG = config.wynd;

const RestHost = CONFIG.rest_host;
const RestPath = CONFIG.rest_path;
const REQUESTED_SYMBOLS = CONFIG.symbols;

export class WyndDexProvider implements Provider {
    name: string;

    constructor() {
        this.name = "WyndDex (Juno)";
    }

    isEnabled(): boolean {
        return CONFIG.enabled;
    }

    async getPrices(): Promise<Data[]> {
        const res = await fetch(`${RestHost}${RestPath}`);
        const data = await res.json();                
        let data_arr: Data[] = [];

        for(const price of data) {
            for(const [k, v] of Object.entries(REQUESTED_SYMBOLS)) {
                // loop through v
                for(const ticker of v) {
                    if(price.asset === ticker) {
                        data_arr.push({
                            id: k,
                            value: Math.round(Number(price.priceInUsd) * 10 ** 6)
                        });
                    }
                }
            }
        }

        return data_arr;
    }
}