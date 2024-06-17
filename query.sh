PROXY=https://devnet-gateway.multiversx.com
USER1=erd1wn0y9x0yeurxsf8396xdpltc0ptakcttk6nffclccka5hxm4qjtsfnzgcs
USER2=erd1d9zt6da8qk2arrzv3zw5rh9wya6uag5xp2qsn6ugwfqdtg8845ps0978d6
CONTRACT=erd1qqqqqqqqqqqqqpgq2xvvvnrmc0w05mu030mmmsj54scsuq4x835s0ulmnc

echo "AirDrops en ligne"
mxpy contract query $CONTRACT --proxy $PROXY --function getAirdrops

echo "Liste des comptes"
mxpy contract query $CONTRACT --proxy $PROXY --function getTokens

echo "Etats des comptes"
mxpy account get --proxy $PROXY --address $USER1
mxpy account get --proxy $PROXY --address $USER2
