#!/bin/bash 
# Input: arg0 = B, arg1 = C, arg2 = key
# Output: A (Execute A = B + C)

# Find key using:
# docker exec -it sabre-cli cat /root/.sawtooth/keys/root.pub

# Init value to B and C
docker exec -it sabre-shell bash -c "intkey set B $1 --url http://rest-api:9708 ; intkey set C $2 --url http://rest-api:9708 ; intkey list --url http://rest-api:9708"

# Generate the Payload File
docker exec -it intkey-add-cli bash -c "cd /project/example/intkey_add/cli ; \
  intkey-add add A B C --output payload"

# Create a Contract Registry
# Upload the Contract Definition File
# Create a Namespace Registry and Set Contract Permissions
# Execute the Smart Contract
docker exec -it sabre-cli bash -c "sabre cr --create intkey_add --owner $3 --url http://rest-api:9708 ; \
 sabre upload --filename ../example/intkey_add/intkey_add.yaml --url http://rest-api:9708 ; \
 sabre ns --create 1cf126 --owner $3 --url http://rest-api:9708 ; sabre perm 1cf126 intkey_add --read --write --url http://rest-api:9708 ; \
 sabre ns --create cad11d --owner $3 --url http://rest-api:9708 ; sabre perm cad11d intkey_add --read --url http://rest-api:9708 ; \
 sabre exec --contract intkey_add:1.0 --payload /project/example/intkey_add/cli/payload --inputs 1cf126 --inputs cad11d --outputs 1cf126 --url http://rest-api:9708"

# wait for a moment
sleep 2

# Check the result value: It should be A
docker exec -it sabre-shell intkey list --url http://rest-api:9708

