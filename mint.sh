#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
COLLECTION=0x546f6b656d6f6e
TICKER=0x544f4b454d4f4e
PROXY=https://devnet-gateway.multiversx.com
TOTAL="50000000000000000"


echo "Mint de la collection"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/owner.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --value $TOTAL \
        --gas-limit 500000000 \
        --function issue_token \
        --arguments $COLLECTION $TICKER \
        --send \
        --outfile="./output/create-collection.json"

TRANSACTION=$(mxpy data parse --file="./output/create-collection.json" --expression="data['emittedTransactionHash']")
echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"