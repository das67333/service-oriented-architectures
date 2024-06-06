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

    FIRST_NAME=Ben
    LAST_NAME=Big
    BIRTH_DATE="2012-04-23"
    EMAIL=hcenquiries@parliament.uk
    PHONE="0800 112 4272"

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

    response=$(curl -s -X PUT $HOST/profile -H "$ST" -H "$SJ" -w "%{http_code}" \
            -d "{ \"first_name\": \"$FIRST_NAME\", \"last_name\": \"$LAST_NAME\",
            \"birth_date\": \"$BIRTH_DATE\", \"email\": \"$EMAIL\",
            \"phone\": \"$PHONE\" }")
    if [[ $response != "200" ]]; then
        echo "Update user data failed" && false
    fi

    CONTENT="Hello world!"
    POST_ID=$(curl -s -X POST $HOST/post -H "$ST" -H "$SJ" \
            -d "{ \"content\": \"$CONTENT\" }" | jq -r ".post_id")
    if [[ -z "$POST_ID" ]]; then
        echo "Create post failed" && false
    fi

    response=$(curl -s -X GET $HOST/post/$POST_ID | jq -r ".content")
    if [[ $response != $CONTENT ]]; then
        echo "Get post failed" && false
    fi

    response=$(curl -s -X GET "$HOST/posts?login=$LOGIN&start_id=1&count=100" | jq -r ".[0].content")
    if [[ $response != $CONTENT ]]; then
        echo "Get all posts failed" && false
    fi

    response=$(curl -s -X GET "$HOST/posts?login=$LOGIN&start_id=1&count=2" | jq -r ".[0].content")
    if [[ $response != $CONTENT ]]; then
        echo "Get first 2 posts failed" && false
    fi

    response=$(curl -s -X GET "$HOST/posts?login=alien&start_id=1&count=100" -w "%{http_code}")
    if [[ ${response: -3} != "404" ]]; then
        echo "Get posts of non-existent user should return 404" && false
    fi

    CONTENT="Let's go"
    response=$(curl -s -X PUT $HOST/post/$POST_ID -H "$ST" -H "$SJ" -w "%{http_code}" \
            -d "{ \"content\": \"$CONTENT\" }")
    if [[ $response != "200" ]]; then
        echo "Update post failed" && false
    fi

    response=$(curl -s -X PUT "$HOST/post/$((POST_ID+1))" -H "$ST" -H "$SJ" -w "%{http_code}" \
            -d "{ \"content\": \"CONTENT\" }")
    if [[ ${response: -3} != "404" ]]; then
        echo "Update non-existent post should return 404" && false
    fi

    response=$(curl -s -X GET $HOST/post/$POST_ID | jq -r ".content")
    if [[ $response != $CONTENT ]]; then
        echo "Get post failed" && false
    fi

    response=$(curl -s -X DELETE $HOST/post/$POST_ID -H "$ST" -w "%{http_code}")
    if [[ $response != "200" ]]; then
        echo "Remove post failed" && false
    fi

    response=$(curl -s -X GET $HOST/post/$POST_ID -w "%{http_code}")
    if [[ ${response: -3} != "404" ]]; then
        echo "Get non-existent post should return 404" && false
    fi

    response=$(curl -s -X DELETE $HOST/post/$POST_ID -H "$ST" -w "%{http_code}")
    if [[ ${response: -3} != "404" ]]; then
        echo "Remove non-existent post should return 404" && false
    fi
}

set_up
run
