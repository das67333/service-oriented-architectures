#!/bin/bash

HOST=localhost:3001
SJ="Content-Type: application/json"

LOGIN=ben
PASSWORD=big

FIRST_NAME=Ben
LAST_NAME=Big
BIRTH_DATE="2012-04-23"
EMAIL=hcenquiries@parliament.uk
PHONE="0800 112 4272"

curl -s -X POST $HOST/signup -H "$SJ" -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }"
echo Signup

TOKEN=$(curl -s -X POST $HOST/login -H "$SJ" -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }" | jq -r ".token")
echo "Login, TOKEN=$TOKEN"
ST="Authorization: $TOKEN"

curl -s -X PUT $HOST/profile -H "$ST" -H "$SJ" -d "{ \"first_name\": \"$FIRST_NAME\", \"last_name\": \"$LAST_NAME\", \"birth_date\": \"$BIRTH_DATE\", \"email\": \"$EMAIL\", \"phone\": \"$PHONE\" }"
echo "Updated user data"

# create posts
for CONTENT in "Hello," "wide" "world," "go"; do
    POST_ID=$(curl -s -X POST $HOST/post -H "$ST" -H "$SJ" -d "{ \"content\": \"$CONTENT\" }" | jq -r ".post_id")
    echo "Created post with ID=$POST_ID and CONTENT=\"$CONTENT\""
done

# update post
curl -s -X PUT $HOST/post/$POST_ID -H "$ST" -H "$SJ" -d "{ \"content\": \"GGo\" }"
echo "Updated post with ID=$POST_ID"

echo -n "Trying to update non-existent post with ID=$((POST_ID+1)): "
curl -s -X PUT "$HOST/post/$((POST_ID+1))" -H "$ST" -H "$SJ" -d "{ \"content\": \"GGo\" }"
echo

# get post
echo -n "Receive post with ID=$POST_ID: "
curl -s -X GET $HOST/post/$POST_ID
echo

# remove post
curl -s -X DELETE $HOST/post/$POST_ID -H "$ST"
echo "Removed post with ID=$POST_ID"

echo -n "Trying to remove post with ID=$POST_ID again: "
curl -s -X DELETE $HOST/post/$POST_ID -H "$ST"
echo

echo -n "Trying to receive post with ID=$POST_ID again: "
curl -s -X GET $HOST/post/$POST_ID
echo

echo "Receive all posts of current user:"
curl -s -X GET "$HOST/posts?login=$LOGIN&start_id=1&count=100"
echo

echo "First 2 posts:"
curl -s -X GET "$HOST/posts?login=$LOGIN&start_id=1&count=2"
echo

echo -n "Receive all posts of non-existent user: "
curl -s -X GET "$HOST/posts?login=alien&start_id=1&count=100"
echo

curl -s -X POST "$HOST/post/$((POST_ID-1))/view" -H "$ST"
echo "Viewed post with ID=$((POST_ID-1))"

curl -s -X POST "$HOST/post/$POST_ID/like" -H "$ST"
echo "Liked post with ID=$POST_ID"

echo -n "Check stats service response: "
curl -s -X GET http://localhost:3101/ -w "{http_code: %{http_code}}"
