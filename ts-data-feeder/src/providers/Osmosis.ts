import { Data, Provider, Average, Averages } from '../types';

// https://api-osmosis.imperator.co/swagger/
// config
const REQUESTED_SYMBOLS = ["ATOM", "OSMO", "JUNO"]

const RestHost = "https://api-osmosis.imperator.co";
const RestPath = "/tokens/v2/%TICKER%"

export class OsmosisProvider implements Provider {
    name: string;

    constructor() {
        this.name = "Osmosis DEX";
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