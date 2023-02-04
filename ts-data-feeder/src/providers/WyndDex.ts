import { Data, Provider, Average, Averages } from '../types';

// config
const REQUESTED_SYMBOLS = { 
    "JUNO": ["ujuno"],  
    "OSMO": ["ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518"], 
    "ATOM": ["ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9"]
}

const RestHost = "https://api.wynddao.com";
const RestPath = "/assets/prices"

export class WyndDexProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Coinbase";
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