PROXY=https://devnet-gateway.multiversx.com
USER1=$(mxpy wallet convert --infile ./wallet/user1.pem --in-format pem --out-format address-bech32)
USER2=$(mxpy wallet convert --infile ./wallet/user2.pem --in-format pem --out-format address-bech32)
USER3=$(mxpy wallet convert --infile ./wallet/user3.pem --in-format pem --out-format address-bech32)
USER4=$(mxpy wallet convert --infile ./wallet/user4.pem --in-format pem --out-format address-bech32)
AIRDROPPER=$(mxpy wallet convert --infile ./wallet/airdropper.pem --in-format pem --out-format address-bech32)
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
TOTAL="40000000000000000"
CLAIM="10000000000000000"
START="1000"
DURATION="60000"

echo "Airdrop de egld"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/airdropper.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --value $TOTAL \
        --gas-limit 50000000 \
        --function fund \
        --arguments $CLAIM $START $DURATION 0 $USER1 $USER2 $USER3 \
        --send \
        --outfile="./output/transaction-fund-open.json"

TRANSACTION=$(mxpy data parse --file="./output/transaction-fund.json" --expression="data['emittedTransactionHash']")
echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"

#
#echo "Airdrop d'egld ?"
#sleep 10
#mxpy --verbose contract call $CONTRACT \
#        --pem ./wallet/airdropper.pem --recall-nonce \
#        --proxy $PROXY --chain D \
#        --value 4000000000000000 \
#        --gas-limit 50000000 \
#        --function fund \
#        --arguments "1000000000000000" "0" "100000" $USER1 $USER2 $USER3 \
#        --send \
#        --outfile="./output/transaction-fund.json"
#
#TRANSACTION=$(mxpy data parse --file="./output/transaction-fund.json" --expression="data['emittedTransactionHash']")
#echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"