echo "Construction"
mxpy contract build

PROXY=https://devnet-gateway.multiversx.com

echo "Déploiement du contract"
mxpy contract deploy --metadata-payable --metadata-not-upgradeable --recall-nonce \
        --bytecode=./output/tokemonworld.wasm \
        --pem=./wallet/owner.pem \
        --gas-limit 60000000 \
        --proxy $PROXY --chain D \
        --arguments 500 \
        --send \
        --outfile=./output/deploy-devnet.interaction.json


TRANSACTION=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")

echo "https://devnet-explorer.multiversx.com/accounts/$CONTRACT"