# ========================
# === Helper Functions ===
# ========================
function query_contract {
    $BINARY query wasm contract-state smart $1 $2 --output json
}

function wasm_cmd {
    CONTRACT=$1
    MESSAGE=$2
    FUNDS=$3
    SHOW_LOG=${4:dont_show}
    ARGS=${5:-$JUNOD_COMMAND_ARGS}
    echo "EXECUTE $MESSAGE on $CONTRACT"

    # if length of funds is 0, then no funds are sent
    if [ -z "$FUNDS" ]; then
        FUNDS=""
    else
        FUNDS="--amount $FUNDS"
        echo "FUNDS: $FUNDS"
    fi
    
    # echo "ARGS: $ARGS"

    tx_hash=$($BINARY tx wasm execute $CONTRACT $MESSAGE $FUNDS $ARGS | jq -r '.txhash')
    export CMD_LOG=$($BINARY query tx $tx_hash --output json | jq -r '.raw_log')    
    if [ "$SHOW_LOG" == "show_log" ]; then
        echo -e "raw_log: $CMD_LOG\n================================\n"
    fi    
}

# CW721
function mint_cw721 {
    CONTRACT_ADDR=$1
    TOKEN_ID=$2
    OWNER=$3
    TOKEN_URI=$4
    EXECUTED_MINT_JSON=`printf '{"mint":{"token_id":"%s","owner":"%s","token_uri":"%s"}}' $TOKEN_ID $OWNER $TOKEN_URI`
    TXMINT=$($BINARY tx wasm execute "$CONTRACT_ADDR" "$EXECUTED_MINT_JSON" $JUNOD_COMMAND_ARGS | jq -r '.txhash') && echo $TXMINT
}

function send_nft_to_listing {
    INTERATING_CONTRACT=$1
    CW721_CONTRACT_ADDR=$2
    TOKEN_ID=$3
    LISTING_ID=$4
    
    NFT_LISTING_BASE64=`printf '{"add_to_listing_cw721":{"listing_id":"%s"}}' $LISTING_ID | base64 -w 0`    
    SEND_NFT_JSON=`printf '{"send_nft":{"contract":"%s","token_id":"%s","msg":"%s"}}' $INTERATING_CONTRACT $TOKEN_ID $NFT_LISTING_BASE64`        

    wasm_cmd $CW721_CONTRACT_ADDR "$SEND_NFT_JSON" "" show_log
}

# CW20 Tokens
function send_cw20_to_listing {
    INTERATING_CONTRACT=$1
    CW20_CONTRACT_ADDR=$2
    AMOUNT=$3
    LISTING_ID=$4
    
    LISTING_BASE64=`printf '{"add_funds_to_sale_cw20":{"listing_id":"%s"}}' $LISTING_ID | base64 -w 0`               
    SEND_TOKEN_JSON=`printf '{"send":{"contract":"%s","amount":"%s","msg":"%s"}}' $INTERATING_CONTRACT $AMOUNT $LISTING_BASE64`        

    wasm_cmd $CW20_CONTRACT_ADDR "$SEND_TOKEN_JSON" "" show_log
}