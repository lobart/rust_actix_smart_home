#!/bin/bash
first_house_id=$(curl -d '{"name":"FirstHouse"}' -X POST -H "Content-Type: application/json" http://localhost:8080/house | jq -r '.id')
second_house_id=$(curl -d '{"name":"SecondHouse"}' -X POST -H "Content-Type: application/json" http://localhost:8080/house | jq -r '.id')

first_room_id_1=$(curl -d '{"name":"FirstRoom", "house":"'$first_house_id'"}' \
                        -H "Content-Type: application/json" \
                        -X POST http://localhost:8080/room | jq -r '.id')

curl -d '{"name":"FirstDevice", "room":"'$first_room_id_1'",
         "typ":"SmartSocket", "state":"true",
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device

curl -d '{"name":"SecondDevice", "room":"'$first_room_id_1'", 
         "typ":"SmartThermometer", "state":"true", 
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device 

second_room_id_1=$(curl -d '{"name":"SecondRoom", "house":"'$first_house_id'"}' \
                        -H "Content-Type: application/json" \
                        -X POST http://localhost:8080/room | jq -r '.id')

curl -d '{"name":"FirstDevice", "room":"'$second_room_id_1'", 
         "typ":"SmartSocket", "state":"true", 
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device 

curl -d '{"name":"SecondDevice", "room":"'$second_room_id_1'", 
         "typ":"SmartThermometer", "state":"true",  
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device

first_room_id_2=$(curl -d '{"name":"FirstRoom1", "house":"'$second_house_id'"}' \
                        -H "Content-Type: application/json" \
                        -X POST http://localhost:8080/room | jq -r '.id')

curl -d '{"name":"FirstDevice", "room":"'$first_room_id_2'", 
         "typ":"SmartSocket", "state":"true",  
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device

curl -d '{"name":"SecondDevice", "room":"'$first_room_id_2'", 
         "typ":"SmartThermometer", "state":"true", 
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device

second_room_id_2=$(curl -d '{"name":"SecondRoom", "house":"'$second_house_id'"}' \
                        -H "Content-Type: application/json" \
                        -X POST http://localhost:8080/room | jq -r '.id')

curl -d '{"name":"FirstDevice", "room":"'$second_room_id_2'", 
         "typ":"SmartSocket", "state":"true",  
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device

curl -d '{"name":"SecondDevice", "room":"'$second_room_id_2'", 
         "typ":"SmartThermometer", "state":"true", 
         "variable":0, "address":"192.168.0.1"}' \
         -H "Content-Type: application/json" \
          -X POST http://localhost:8080/device