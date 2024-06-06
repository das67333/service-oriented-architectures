#!/bin/bash

set_up() {
    # interrupt the script if any command returns a non-zero exit code
    set -e
    # call tear_down function on exit
    trap tear_down EXIT

    cd ..
    docker-compose up --build -d
    sleep 5
}

tear_down() {
    set +e
    docker-compose down
}

run() {
    # print the line number of the failed command
    trap 'echo "Test failed on line $LINENO"' ERR

    HOST=localhost:3001
    SJ="Content-Type: application/json"

    # if there is a command-line argument, then use it as LOGIN; if no, use ben
    if [ -n "$1" ]; then
        LOGIN=$1
    else
        LOGIN=ben
    fi
    PASSWORD=big

    response=$(curl -s -X POST $HOST/signup -H "$SJ" -w "%{http_code}" \
            -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }")
    if [[ $response != "200" ]]; then
        echo "Failed to sign up" && false
    fi

    TOKEN=$(curl -s -X POST $HOST/login -H "$SJ" \
            -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }" \
            | jq -r ".token")
    echo "TOKEN=$TOKEN"
    if [[ -z "$TOKEN" ]]; then
        echo "Failed to login" && false
    fi
    ST="Authorization: $TOKEN"

    # simple pseudo-random number generator
    x=1234567890
    for i in {1..7}; do
        CONTENT=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1)
        POST_ID=$(curl -s -X POST $HOST/post -H "$ST" -H "$SJ" \
                -d "{ \"content\": \"$CONTENT\" }" | jq -r ".post_id")
        if [[ -z "$POST_ID" ]]; then
            echo "Failed to create post" && false
        fi

        x=$((x * 48271 % 2147483647))
        for i in $(seq 1 $((x%20))); do
            curl -s -X POST "$HOST/post/$POST_ID/view" -H "$ST"
        done
        x=$((x * 48271 % 2147483647))
        for i in $(seq 1 $((x%20))); do
            curl -s -X POST "$HOST/post/$POST_ID/like" -H "$ST"
        done
    done

    sleep 5

    response=$(curl -s -X GET "$HOST/stats/post/$POST_ID" | jq -c '[.views, .likes]')
    if [[ $response != "[15,12]" ]]; then
        echo "Get post stats failed" && false
    fi

    response=$(curl -s -X GET "$HOST/stats/top_posts/views"  | jq -c '[.[] | .id]')
    if [[ $response != "[4,7,3,5,6]" ]]; then
        echo "Get top posts by views failed" && false
    fi

    response=$(curl -s -X GET "$HOST/stats/top_posts/likes"  | jq -c '[.[] | .id]')
    if [[ $response != "[2,4,6,7,5]" ]]; then
        echo "Get top posts by likes failed" && false
    fi

    response=$(curl -s -X GET "$HOST/stats/top_users" | jq -c '[.[] | .likes]')
    if [[ $response != "[72]" ]]; then
        echo "Get top users by likes failed" && false
    fi
}

set_up
run
