echo "Construction"
mxpy contract build

PROXY=https://devnet-gateway.multiversx.com
ROYALTIES=0
MAX_PROMPT=5

echo "Déploiement du contract"
mxpy contract deploy --metadata-payable --metadata-not-upgradeable --recall-nonce \
        --bytecode=./output/promptmarket.wasm \
        --pem=./wallet/owner.pem \
        --gas-limit 60000000 \
        --argument $ROYALTIES $MAX_PROMPT \
        --proxy $PROXY --chain D \
        --send \
        --outfile=./output/deploy-devnet.interaction.json


TRANSACTION=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")

echo "https://devnet-explorer.multiversx.com/accounts/$CONTRACT"