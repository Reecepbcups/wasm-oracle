export interface Data {
    id: string;
    value: number;
}

export interface Provider {
    name: string;
    getPrices(): Promise<Data[]>;    
    isEnabled(): boolean;
}

export interface Average {
    total: number;
    count: number;
}

export interface Averages {
    [key: string]: Average;
}