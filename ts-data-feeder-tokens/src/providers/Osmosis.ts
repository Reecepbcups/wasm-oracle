import { Data, Provider } from '../types';

// https://api-osmosis.imperator.co/swagger/

import {default as config} from '../../config.json';
const CONFIG = config.osmosis;

const RestHost = CONFIG.rest_host;
const RestPath = CONFIG.rest_path;
const REQUESTED_SYMBOLS = CONFIG.symbols;

export class OsmosisProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Osmosis DEX";
    }

    isEnabled(): boolean {
        return CONFIG.enabled;
    }

    async getPrices(): Promise<Data[]> {
        let data_arr: Data[] = [];
        for(const ticker of REQUESTED_SYMBOLS) {
            
            // https://api-osmosis.imperator.co/tokens/v2/ATOM
            const res = await fetch(`${RestHost}${RestPath.replace("%TICKER%", ticker)}`);
            const data = await res.json();
            // console.log(data);

            data_arr.push({
                id: ticker,
                value: Math.round(Number(data[0].price) * 10 ** 6)
            });                    
        }      

        return data_arr;
    }
}