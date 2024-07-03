#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
INFERENCE=30
MODEL=1
SCALE=200
PROXY=https://devnet-gateway.multiversx.com
USER1=$(mxpy wallet convert --infile ./wallet/user1.pem --in-format pem --out-format address-bech32)
USER2=$(mxpy wallet convert --infile ./wallet/user2.pem --in-format pem --out-format address-bech32)
USER3=$(mxpy wallet convert --infile ./wallet/user3.pem --in-format pem --out-format address-bech32)
USER4=$(mxpy wallet convert --infile ./wallet/user4.pem --in-format pem --out-format address-bech32)
TOKEN="AIRDROP-bc8a67"
PRICE=20
TITLE=0x7061726973

echo "Ajout d'un prompt"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/user1.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --gas-limit 40081355 \
        --function add_server \
        --arguments $TITLE $MODEL $INFERENCE $SCALE $PRICE $TOKEN \
        --send \
        --outfile="./output/add_server.json"

TRANSACTION=$(mxpy data parse --file="./output/add_server.json" --expression="data['emittedTransactionHash']")
echo "add_arena https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"