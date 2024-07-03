#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
TEXT=0x7061726973
SERVER=1
PROXY=https://devnet-gateway.multiversx.com
TOKEN=AIRDROP-bc8a67
TOTAL="10000000000000000000"

echo "Ajout d'un prompt"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/user1.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --gas-limit 500000000 \
        --token-transfers $TOKEN $TOTAL \
        --function add_prompt \
        --arguments $TEXT $SERVER  \
        --send \
        --outfile="./output/servers.json"

TRANSACTION=$(mxpy data parse --file="./output/servers.json" --expression="data['emittedTransactionHash']")
echo "add_arena https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"