PROXY=https://devnet-gateway.multiversx.com
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
CLAIMER=user4

mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/$CLAIMER.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --value 0 \
        --gas-limit 5000000 \
        --function claim \
        --arguments 1 \
        --send \
        --outfile="./output/transaction-claim.json"

TRANSACTION=$(mxpy data parse --file="./output/transaction-claim.json" --expression="data['emittedTransactionHash']")
echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"