#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
PROXY=https://devnet-gateway.multiversx.com
ARENA_ID=1
TOKEMON_ID=1

echo "Entrer dans l'arene $ARENA_ID avec le tokemon $TOKEMON_ID"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/user1.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --gas-limit 500000000 \
        --function goto_arena \
        --arguments $ARENA_ID $TOKEMON_ID \
        --send \
        --outfile="./output/fight.json"

TRANSACTION=$(mxpy data parse --file="./output/fight.json" --expression="data['emittedTransactionHash']")
echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"