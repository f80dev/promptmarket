PROXY=https://devnet-gateway.multiversx.com
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
USER1=$(mxpy wallet convert --infile ./wallet/user1.pem --in-format pem --out-format address-bech32)
USER2=$(mxpy wallet convert --infile ./wallet/user2.pem --in-format pem --out-format address-bech32)
USER3=$(mxpy wallet convert --infile ./wallet/user3.pem --in-format pem --out-format address-bech32)
USER4=$(mxpy wallet convert --infile ./wallet/user4.pem --in-format pem --out-format address-bech32)
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")

#voir https://docs.multiversx.com/sdk-and-tools/sdk-py/mxpy-cli/#querying-a-smart-contract

echo "Evaluation de la marge"
mxpy --verbose contract query $CONTRACT \
        --proxy $PROXY \
        --function getMarge \

echo "Récupération des airdrops"
mxpy --verbose contract query $CONTRACT \
        --proxy $PROXY \
        --function check_airdrop \
        --arguments 1 $USER1