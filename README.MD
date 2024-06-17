# build
    
    mkdir ./wallet
    mxpy wallet new --format pem --outfile=./wallet/sc-owner.pem
    mxpy wallet new --format pem --outfile=./wallet/user1.pem
    mxpy wallet new --format pem --outfile=./wallet/user2.pem
    mxpy wallet new --format pem --outfile=./wallet/user3.pem
    mxpy wallet new --format pem --outfile=./wallet/user4.pem
    mxpy wallet new --format pem --outfile=./wallet/airdropper.pem

pour le remplir, on puet utiliser 
https://r3d4.fr/faucet
https://devnet-wallet.multiversx.com/unlock/pem

# Installation

Déploiement du smart contract:

voir https://docs.multiversx.com/sdk-and-tools/sdk-py/mxpy-cli/#deploying-a-smart-contract


Target devnet:
    mxpy contract build
    mxpy contract deploy --bytecode ./output/tokemonworld.wasm --proxy=https://devnet-gateway.multiversx.com --recall-nonce --arguments 0 --gas-limit 50000000 --pem=./wallet/sc-owner.pem --send

