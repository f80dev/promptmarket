#utiliser https://string-functions.com/string-hex.aspx
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")
VISUAL=0x68747470733a2f2f756e73706c6173682e636f6d2f66722f70686f746f732f756e2d6a65756e652d686f6d6d652d706f7274616e742d756e652d70616972652d64652d6c756e65747465732d64652d736f6c65696c2d726f6e6465732d6b744d696b375167523049
NAME=0x6361726170756365
PROXY=https://devnet-gateway.multiversx.com
TOKEN=WATER-09794f
TOTAL="4000"

echo "Mint de la collection"
mxpy --verbose contract call $CONTRACT \
        --pem ./wallet/user1.pem --recall-nonce \
        --proxy $PROXY --chain D \
        --gas-limit 500000000 \
        --token-transfers $TOKEN $TOTAL \
        --function make_nft \
        --arguments $NAME $VISUAL \
        --send \
        --outfile="./output/make.json"

TRANSACTION=$(mxpy data parse --file="./output/make.json" --expression="data['emittedTransactionHash']")
echo "https://devnet-explorer.multiversx.com/transactions/$TRANSACTION"