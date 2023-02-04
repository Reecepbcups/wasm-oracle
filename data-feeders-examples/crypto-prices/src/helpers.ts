import { Coin, coin, GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { Secp256k1HdWallet, SigningCosmWasmClient } from "cosmwasm";
import { rpcEndpoint } from "./app";

export const getBalance = async (wallet_addr: any, denom: string) => {
    // get craft escrow account
    let balance = coin("0", denom);
    try {
        const client = await SigningStargateClient.connectWithSigner(rpcEndpoint, wallet_addr);        
        balance = await client.getBalance(wallet_addr, denom)        
    } catch (error) {
        console.log("getBalance", error);
    }
    return balance;
}


export const getAccountFromMnemonic = async (mnemonic: any, prefix: string = "cosmos") => {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix });
    const [account] = await wallet.getAccounts();
    return {
        wallet: wallet,
        account: account,
    }
}

export const setupCWClient = async (mnemonic: string, denom: string, prefix: string) => {
    const gas = GasPrice.fromString(`0.03${denom}`);
    const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: prefix });
    const client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, { gasPrice: gas });
    return client;
}

export const getRandomAccount = async (prefix: string = "cosmos") => {
    let wallet = await Secp256k1HdWallet.generate(12, { prefix: prefix });
    const [account] = await wallet.getAccounts();
    return {
        wallet: wallet,
        account: account
    }
};


export const sendTokensToAccount = async (client: any, data: any, to_address: string, rpc: string, coins: [Coin], fee: any) => {
    const result = await client.sendTokens(data.account.address, to_address, coins, fee);
    return result;
}