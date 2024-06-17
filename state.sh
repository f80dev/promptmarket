PROXY=https://devnet-gateway.multiversx.com
CONTRACT=$(mxpy data parse --file="./output/deploy-devnet.interaction.json" --expression="data['contractAddress']")

echo "Etat du contrat $CONTRACT"

echo "AirDrops en ligne"
mxpy contract query $CONTRACT --proxy $PROXY --function getAirdrops

echo "Claimer eligible"
mxpy contract query $CONTRACT --proxy $PROXY --function getAddresses

echo "Liste des tokens"
mxpy contract query $CONTRACT --proxy $PROXY --function getTokens

echo "Etats des comptes de $USER1"

for i in 1 2 3 4 ; do
  echo "user$i"
  rm ./output/user$i.txt
  mxpy wallet convert --infile ./wallet/user$i.pem --in-format pem --out-format address-bech32 --outfile ./output/user$i.txt
  mxpy account get --proxy $PROXY --address $(cat ./output/user$i.txt)
  mxpy contract query $CONTRACT --proxy $PROXY --function hasAirdrop --arguments $(cat ./output/user$i.txt)
done
