#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
ARENA_LON=10
ARENA_LAT=10
ARENA_NAME=0x7061726973
PROXY=https://devnet-gateway.multiversx.com
TOKEN=WATER-09794f
TOTAL="4000"

echo "Mint de la collection"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/owner.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --gas-limit 500000000 \
        --function add_arena \
        --arguments $ARENA_LON $ARENA_LAT $ARENA_NAME \
        --send \
        --outfile="./output/add_arena.json"

TRANSACTION=$(mxpy data parse --file="./output/add_arena.json" --expression="data['emittedTransactionHash']")
echo "add_arena https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"