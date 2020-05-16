#!/bin/bash 
docker-compose -f docker-compose.yaml -f example/intkey_add/docker-compose.yaml down
docker-compose -f docker-compose.yaml -f example/intkey_add/docker-compose.yaml up