# Test script for Juno Smart Contracts (By @Reecepbcups)
# ./github/workflows/e2e.yml
#
# sh ./e2e/test_e2e.sh
#
# NOTES: anytime you use jq, use `jq -rc` for ASSERT_* functions (-c removes format, -r is raw to remove \" quotes)

# get functions from helpers file 
# -> query_contract, wasm_cmd, mint_cw721, send_nft_to_listing, send_cw20_to_listing
source ./e2e/helpers.sh

CONTAINER_NAME="juno_oracle_contract"
BINARY="docker exec -i $CONTAINER_NAME junod"
DENOM='ujunox'
JUNOD_CHAIN_ID='testing'
JUNOD_NODE='http://localhost:26657/'
# globalfee will break this in the future
TX_FLAGS="--gas-prices 0.1$DENOM --gas-prices="0ujunox" --gas 5000000 -y -b block --chain-id $JUNOD_CHAIN_ID --node $JUNOD_NODE --output json"
export JUNOD_COMMAND_ARGS="$TX_FLAGS --from test-user"
export KEY_ADDR="juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl"


# ===================
# === Docker Init ===
# ===================
function stop_docker {
    docker kill $CONTAINER_NAME
    docker rm $CONTAINER_NAME
    docker volume rm --force junod_data
}

function start_docker {
    IMAGE_TAG=${2:-"12.0.0-alpha3"}
    BLOCK_GAS_LIMIT=${GAS_LIMIT:-100000000} # mirrors mainnet

    echo "Building $IMAGE_TAG"
    echo "Configured Block Gas Limit: $BLOCK_GAS_LIMIT"

    stop_docker    

    # run junod docker
    docker run --rm -d --name $CONTAINER_NAME \
        -e STAKE_TOKEN=$DENOM \
        -e GAS_LIMIT="$GAS_LIMIT" \
        -e UNSAFE_CORS=true \
        -e TIMEOUT_COMMIT="500ms" \
        -p 1317:1317 -p 26656:26656 -p 26657:26657 \
        --mount type=volume,source=junod_data,target=/root \
        ghcr.io/cosmoscontracts/juno:$IMAGE_TAG /opt/setup_and_run.sh $KEY_ADDR    
}

function compile_and_copy {    
    # compile vaults contract here
    docker run --rm -v "$(pwd)":/code \
      --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
      --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
      cosmwasm/rust-optimizer:0.12.11

    # copy wasm to docker container
    docker cp ./artifacts/oracle.wasm $CONTAINER_NAME:/oracle.wasm
}

function health_status {
    # validator addr
    VALIDATOR_ADDR=$($BINARY keys show validator --address) && echo "Validator address: $VALIDATOR_ADDR"

    BALANCE_1=$($BINARY q bank balances $VALIDATOR_ADDR) && echo "Pre-store balance: $BALANCE_1"

    echo "Address to deploy contracts: $KEY_ADDR"
    echo "JUNOD_COMMAND_ARGS: $JUNOD_COMMAND_ARGS"
}

# ========================
# === Contract Uploads ===
# ========================
function upload_oracle {
    echo "Storing contract..."
    UPLOAD=$($BINARY tx wasm store /oracle.wasm $JUNOD_COMMAND_ARGS | jq -r '.txhash') && echo $UPLOAD
    BASE_CODE_ID=$($BINARY q tx $UPLOAD --output json | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value') && echo "Code Id: $BASE_CODE_ID"

    # == INSTANTIATE ==
    ADMIN="$KEY_ADDR"

    JSON_MSG=$(printf '{"addresses":["%s","%s"],"denoms":["JUNO"]}' "$ADMIN" "juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk")
    TX_HASH=$($BINARY tx wasm instantiate "$BASE_CODE_ID" $JSON_MSG --label "vault" $JUNOD_COMMAND_ARGS --admin $KEY_ADDR | jq -r '.txhash') && echo $VAULT_TX


    export ORACLE_CONTRACT=$($BINARY query tx $TX_HASH --output json | jq -r '.logs[0].events[0].attributes[0].value') && echo "ORACLE_CONTRACT: $ORACLE_CONTRACT"
}

# ===============
# === ASSERTS ===
# ===============
FINAL_STATUS_CODE=0

function ASSERT_EQUAL {
    # For logs, put in quotes. 
    # If $1 is from JQ, ensure its -r and don't put in quotes
    if [ "$1" != "$2" ]; then        
        echo "ERROR: $1 != $2" 1>&2
        FINAL_STATUS_CODE=1 
    else
        echo "SUCCESS"
    fi
}

function ASSERT_CONTAINS {
    if [[ "$1" != *"$2"* ]]; then
        echo "ERROR: $1 does not contain $2" 1>&2        
        FINAL_STATUS_CODE=1 
    else
        echo "SUCCESS"
    fi
}

function add_accounts {
    # provision juno default user i.e. juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl
    echo "decorate bright ozone fork gallery riot bus exhaust worth way bone indoor calm squirrel merry zero scheme cotton until shop any excess stage laundry" | $BINARY keys add test-user --recover --keyring-backend test
    # juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk
    echo "wealth flavor believe regret funny network recall kiss grape useless pepper cram hint member few certain unveil rather brick bargain curious require crowd raise" | $BINARY keys add other-user --recover --keyring-backend test

    # send some 10 junox funds to the user
    $BINARY tx bank send test-user juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk 10000000ujunox $JUNOD_COMMAND_ARGS

    # check funds where sent
    other_balance=$($BINARY q bank balances juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk --output json)
    ASSERT_EQUAL "$other_balance" '{"balances":[{"denom":"ujunox","amount":"10000000"}],"pagination":{"next_key":null,"total":"0"}}'
}

# === COPY ALL ABOVE TO SET ENVIROMENT UP LOCALLY ====



# =============
# === LOGIC ===
# =============

start_docker
compile_and_copy # the compile takes time for the docker container to start up

sleep 5
# add query here until state check is good, then continue

# Don't allow errors after this point
set -e

health_status
add_accounts

upload_oracle

# ORACLE_CONTRACT=juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8

# == INITIAL TEST ==

addrs=$(query_contract $ORACLE_CONTRACT '{"addresses":{}}' | jq -r '.data.addresses') && echo $addrs
# ASSERT_EQUAL "$admin" $KEY_ADDR

# no price yet
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"median"}}' | jq -r '.data') && echo $price
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"average"}}' | jq -r '.data') && echo $price
# ASSERT_EQUAL "$admin" $KEY_ADDR

# submit price (so $1 is 1_000_000. Then when we query, we just / 1_000_000 = 1)
# only the addresses in 'addresses' can submit prices. 
# TODO: add exponent to the price (6 for juno in this case) = $1USD
wasm_cmd $ORACLE_CONTRACT '{"submit_price":{"denom":"JUNO","price":1000000}}' "" show_log
wasm_cmd $ORACLE_CONTRACT '{"submit_price":{"denom":"JUNO","price":1002500}}' "" show_log "$TX_FLAGS --keyring-backend test --from other-user"
# need a 3rd person to test median
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"median"}}' | jq -r '.data') && echo $price
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"average"}}' | jq -r '.data') && echo $price

# fails, not a good user
# wasm_cmd $ORACLE_CONTRACT '{"submit_price":{"denom":"JUNO","price":69420}}' "" show_log "$TX_FLAGS --keyring-backend test --from other-user"
# price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"median"}}' | jq -r '.data') && echo $price

wasm_cmd $ORACLE_CONTRACT '{"submit_price":{"denom":"JUNO","price":1000005}}' "" show_log
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"median"}}' | jq -r '.data') && echo $price
price=$(query_contract $ORACLE_CONTRACT '{"price":{"denom":"JUNO","measure":"average"}}' | jq -r '.data') && echo $price

# fails, not accepted
wasm_cmd $ORACLE_CONTRACT '{"submit_price":{"denom":"OSMO","price":500000}}' "" show_log

bulk_prices=$(query_contract $ORACLE_CONTRACT '{"wallets_prices":{"address":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl"}}' | jq -r '.data') && echo $bulk_prices
bulk_prices=$(query_contract $ORACLE_CONTRACT '{"wallets_prices":{"address":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk"}}' | jq -r '.data') && echo $bulk_prices


bulk_prices=$(query_contract $ORACLE_CONTRACT '{"wallets_prices":{"address":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl"}}' | jq -r '.data') && echo $bulk_prices


# TODO: add admin (DAO) which can add other denoms to accept

exit 1

# === LISTINGS TEST ===
function test_duplicate_ask_denoms {
    # make a listing with 2 unique but duplicate denoms, ensure the denoms are merged correctly on ask
    # failure to do this = the user can not purchase the listing, even if they sent 2ujunox
    wasm_cmd $ORACLE_CONTRACT '{"create_listing":{"create_msg":{"id":"vault_duplicate","ask":{"native":[{"denom":"ujunox","amount":"1"},{"denom":"ujunox","amount":"1"}],"cw20":[],"nfts":[]}}}}' "1ucosm" show_logs
    ASSERT_CONTAINS "$CMD_LOG" "Duplicates found in Ask Price"

    # ensure it removes and 0 denoms, but does not error if no duplicates
    wasm_cmd $ORACLE_CONTRACT '{"create_listing":{"create_msg":{"id":"vault_c1","ask":{"native":[{"denom":"ujunox","amount":"2"},{"denom":"ujunox","amount":"0"}],"cw20":[],"nfts":[]}}}}' "1ucosm" show_logs    
    asking_values=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_c1"}}' | jq -rc '.data.ask')
    ASSERT_EQUAL "$asking_values" '[["ujunox","2"]]'

    # finalize
    wasm_cmd $ORACLE_CONTRACT '{"finalize":{"listing_id":"vault_c1","seconds":5000}}' "" show_log

    # buy the listing to keep future test clean
    wasm_cmd $ORACLE_CONTRACT '{"create_bucket":{"bucket_id":"buyer_com"}}' "2ujunox" show_log
    wasm_cmd $ORACLE_CONTRACT '{"buy_listing":{"listing_id":"vault_c1","bucket_id":"buyer_com"}}' "" show_log 
    wasm_cmd $ORACLE_CONTRACT '{"withdraw_purchased":{"listing_id":"vault_c1"}}' "" dont_show

    # ensure the listing was removed        
    listings=$(query_contract $ORACLE_CONTRACT '{"get_all_listings":{}}' --output json)       
    ASSERT_EQUAL $listings '{"data":{"listings":[]}}'
}

function test_all_listings {
    # Selling 10ucosm for 5ujunox
    wasm_cmd $ORACLE_CONTRACT '{"create_listing":{"create_msg":{"id":"vault_1","ask":{"native":[{"denom":"ujunox","amount":"5"}],"cw20":[],"nfts":[]}}}}' "10ucosm" show_log

    # Ensure listing went up correctly
    listing_1=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_1"}}')
    ASSERT_EQUAL "$listing_1" '{"data":{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","status":"Being Prepared","for_sale":[["ucosm","10"]],"ask":[["ujunox","5"]],"expiration":"None","whitelisted_buyer":"None"}}'

    # Ensure duplicate vault_id fails
    wasm_cmd $ORACLE_CONTRACT '{"create_listing":{"create_msg":{"id":"vault_1","ask":{"native":[{"denom":"ujunox","amount":"1"}],"cw20":[],"nfts":[]}}}}' "1ujunox"
    ASSERT_CONTAINS "$CMD_LOG" 'ID already taken'

    echo "Sending NFT id 1 to the listing"
    send_nft_to_listing $ORACLE_CONTRACT $CW721_CONTRACT "1" "vault_1"
    query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_1"}}'

    # owner should now be the ORACLE_CONTRACT after sending (We check that the NFT is in the listing after the CW20)
    owner_of_nft=$(query_contract $CW721_CONTRACT '{"all_nft_info":{"token_id":"1"}}' | jq -r '.data.access.owner')
    ASSERT_EQUAL "$owner_of_nft" "$ORACLE_CONTRACT"

    # Send 20 CW20 coin to the listing
    echo "Sending 20 CW20 token to the listing"
    send_cw20_to_listing $ORACLE_CONTRACT $CW20_CONTRACT "20" "vault_1"

    # Ensure the CW20 token & CW721 is now apart of the listing
    # todo: this will fail if the order of the array changes given there is no difference between cw20 and cw721 in it right? or does jq sort deterministically?
    listing_values=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_1"}}' | jq -r '.data.for_sale')
    ASSERT_EQUAL $listing_values `printf '[["ucosm","10"],["%s","20"],["%s":"1"]]' $CW20_CONTRACT $CW721_CONTRACT`

    # Finalize the listing for purchase after everything is added
    wasm_cmd $ORACLE_CONTRACT '{"finalize":{"listing_id":"vault_1","seconds":5000}}' "" show_log
    # try to finalize again, will fail
    wasm_cmd $ORACLE_CONTRACT '{"finalize":{"listing_id":"vault_1","seconds":100}}' ""
    ASSERT_CONTAINS "$CMD_LOG" 'Listing already finalized'

    # Create bucket so we can purchase the listing
    echo "Creating bucket and purchasing listing"
    wasm_cmd $ORACLE_CONTRACT '{"create_bucket":{"bucket_id":"buyer_1"}}' "5ujunox" show_log
    # purchase listing
    wasm_cmd $ORACLE_CONTRACT '{"buy_listing":{"listing_id":"vault_1","bucket_id":"buyer_1"}}' "" show_log
    echo "Withdrawing rewards... (Should do this in buy listing function?)"

    # check users balance changes here after we  execute_withdraw_purchased
    # query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_1"}}' <- ensure it is closed, but I feel when we buy it should auto transfer? Why not?

    wasm_cmd $ORACLE_CONTRACT '{"withdraw_purchased":{"listing_id":"vault_1"}}' "" show_log
    # ensure listings are empty now
    listings=$(query_contract $ORACLE_CONTRACT '{"get_all_listings":{}}' | jq -r '.data.listings')
    ASSERT_EQUAL "$listings" '[]'
}

function test_whitelist {    
    # Selling 25ucosm for 5ujunox
    wasm_cmd $ORACLE_CONTRACT '{"create_listing":{"create_msg":{"id":"vault_2","ask":{"native":[{"denom":"ujunox","amount":"5"}],"cw20":[],"nfts":[]},"whitelisted_buyer":"juno1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq93ryqp"}}}' "25ucosm" show_log
    # Ensure listing went up correctly
    listing_1=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_2"}}')
    ASSERT_EQUAL "$listing_1" '{"data":{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","status":"Being Prepared","for_sale":[["ucosm","25"]],"ask":[["ujunox","5"]],"expiration":"None","whitelisted_buyer":"juno1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq93ryqp"}}'

    # is hidden from market listings, but would be found in the all listings query
    listings=$(query_contract $ORACLE_CONTRACT '{"get_listings_for_market":{"page_num":1}}' | jq -r '.data.listings')
    ASSERT_EQUAL "$listings" '[]'

    # Ensure the listing is in the whitelist only query
    listings=$(query_contract $ORACLE_CONTRACT '{"get_whitelisted_listings":{"address":"juno1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq93ryqp"}}' | jq -rc '.data.listings')
    ASSERT_EQUAL $listings '[{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","id":"vault_2","finalized_time":null,"expiration_time":null,"status":"being_prepared","claimant":null,"whitelisted_buyer":"juno1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq93ryqp","for_sale":{"native":[{"denom":"ucosm","amount":"25"}],"cw20":[],"nfts":[]},"ask":{"native":[{"denom":"ujunox","amount":"5"}],"cw20":[],"nfts":[]}}]'

    # remove whitelisted buyer test
    wasm_cmd $ORACLE_CONTRACT '{"remove_whitelisted_buyer":{"listing_id":"vault_2"}}' "" show_log
    listing_1_change=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_2"}}')
    ASSERT_EQUAL "$listing_1_change" '{"data":{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","status":"Being Prepared","for_sale":[["ucosm","25"]],"ask":[["ujunox","5"]],"expiration":"None","whitelisted_buyer":"None"}}'

    # ensure the address no longer is in the whitelist query
    listings=$(query_contract $ORACLE_CONTRACT '{"get_whitelisted_listings":{"address":"juno1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq93ryqp"}}' | jq -rc '.data.listings')
    ASSERT_EQUAL $listings '[]'
    # and that it is in the market listings query now
    listing_no_whitelist=$(query_contract $ORACLE_CONTRACT '{"get_all_listings":{}}')
    ASSERT_EQUAL "$listing_no_whitelist" '{"data":{"listings":[{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","id":"vault_2","finalized_time":null,"expiration_time":null,"status":"being_prepared","claimant":null,"whitelisted_buyer":null,"for_sale":{"native":[{"denom":"ucosm","amount":"25"}],"cw20":[],"nfts":[]},"ask":{"native":[{"denom":"ujunox","amount":"5"}],"cw20":[],"nfts":[]}}]}}'

    # change whitelisted buyer to correct address
    wasm_cmd $ORACLE_CONTRACT '{"change_whitelisted_buyer":{"listing_id":"vault_2","new_address":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk"}}' "" show_log
    listing_1_change=$(query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_2"}}')
    ASSERT_EQUAL "$listing_1_change" '{"data":{"creator":"juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl","status":"Being Prepared","for_sale":[["ucosm","25"]],"ask":[["ujunox","5"]],"expiration":"None","whitelisted_buyer":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk"}}'

    # finalize just the natives
    wasm_cmd $ORACLE_CONTRACT '{"finalize":{"listing_id":"vault_2","seconds":5000}}' "" show_log

    # try to buy as the incorrect user (test-user) which is not whitelisted
    wasm_cmd $ORACLE_CONTRACT '{"create_bucket":{"bucket_id":"buyer_2"}}' "5ujunox" show_log
    wasm_cmd $ORACLE_CONTRACT '{"buy_listing":{"listing_id":"vault_2","bucket_id":"buyer_2"}}' "" dont_show_log
    ASSERT_CONTAINS "$CMD_LOG" 'Not whitelisted'
    # Buy as the whitelisted person
    wasm_cmd $ORACLE_CONTRACT '{"create_bucket":{"bucket_id":"buyer_3"}}' "5ujunox" show_log "$TX_FLAGS --keyring-backend test --from other-user"
    wasm_cmd $ORACLE_CONTRACT '{"buy_listing":{"listing_id":"vault_2","bucket_id":"buyer_3"}}' "" show_log "$TX_FLAGS --keyring-backend test --from other-user"
    wasm_cmd $ORACLE_CONTRACT '{"withdraw_purchased":{"listing_id":"vault_2"}}' "" show_log "$TX_FLAGS --keyring-backend test --from other-user"
    # ensure there are 0 listings left
    listings=$(query_contract $ORACLE_CONTRACT '{"get_all_listings":{}}' | jq -r '.data.listings')
    ASSERT_EQUAL "$listings" '[]'

    # check if other-user has the ucosm tokens they bought (they would be down 5ujunox as well)
    balance=$($BINARY q bank balances "juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk" --output json)
    ASSERT_CONTAINS "$balance" '{"denom":"ucosm","amount":"25"}'
    ASSERT_CONTAINS "$balance" '{"denom":"ujunox","amount":"9999995"}'
    # no cw20 or nfts to check here :)
}

test_duplicate_ask_denoms
test_whitelist # run before test_whitelist since we check balances here
test_all_listings

# 1 if any of the above test failed, this way it will ensure to X the github
exit $FINAL_STATUS_CODE

# manual queries
# query_contract $ORACLE_CONTRACT '{"get_config":{}}'
# query_contract $ORACLE_CONTRACT '{"get_all_listings":{}}'
# query_contract $ORACLE_CONTRACT '{"get_listing_info":{"listing_id":"vault_1"}}'
# query_contract $ORACLE_CONTRACT '{"get_listings_by_owner":{"owner":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk"}}'
# query_contract $ORACLE_CONTRACT '{"get_buckets":{"bucket_owner":"juno1efd63aw40lxf3n4mhf7dzhjkr453axurv2zdzk"}}'
# query_contract $ORACLE_CONTRACT '{"get_listings_for_market":{"page_num":1}}'