#!/bin/bash 

# $1 = B
# $2 = C
# docker exec -it sabre-cli cat /root/.sawtooth/keys/root.pub
# $3 = key

# Init value to argument 1 and 2
docker exec -it sabre-shell bash -c "intkey set B $1 --url http://rest-api:9708 ; intkey set C $2 --url http://rest-api:9708 ; intkey list --url http://rest-api:9708"

# Generate the Payload File
docker exec -it intkey-add-cli bash -c "cd /project/example/intkey_add/cli ; intkey-add add A B C --output payload"

# Create a Contract Registry
# Upload the Contract Definition File
docker exec -it sabre-cli bash -c "sabre cr --create intkey_add --owner $3 --url http://rest-api:9708 ; sabre upload --filename ../example/intkey_add/intkey_add.yaml --url http://rest-api:9708 ; sabre ns --create 1cf126 --owner $3 --url http://rest-api:9708 ; sabre perm 1cf126 intkey_add --read --write --url http://rest-api:9708 ; sabre ns --create cad11d --owner $3 --url http://rest-api:9708 ; sabre perm cad11d intkey_add --read --url http://rest-api:9708 ; sabre exec --contract intkey_add:1.0 --payload /project/example/intkey_add/cli/payload --inputs 1cf126 --inputs cad11d --outputs 1cf126 --url http://rest-api:9708"

sleep 5

docker exec -it sabre-shell intkey list --url http://rest-api:9708

