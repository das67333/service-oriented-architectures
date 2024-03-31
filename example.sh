LOGIN=ben
PASSWORD=big
FIRST_NAME=Ben
LAST_NAME=Big
BIRTH_DATE="2012-04-23"
EMAIL=hcenquiries@parliament.uk
PHONE="0800 112 4272"

echo Signup
curl -s "localhost:3000/signup" -H "Content-Type: application/json" -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }"
echo

echo Login
TOKEN=$(curl -s -X POST "localhost:3000/login" -H "Content-Type: application/json" -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }" | jq -r ".token")
echo TOKEN=$TOKEN

echo "Update user data"
curl -s -X PUT "localhost:3000/update" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"first_name\": \"$FIRST_NAME\", \"last_name\": \"$LAST_NAME\", \"birth_date\": \"$BIRTH_DATE\", \"email\": \"$EMAIL\", \"phone\": \"$PHONE\" }"

# create posts
POST_ID_1=$(curl -s -X POST "localhost:3000/create_post" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"content\": \"Hello\" }" | jq -r ".post_id")
echo POST_ID_1=$POST_ID_1
POST_ID_2=$(curl -s -X POST "localhost:3000/create_post" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"content\": \"world,\" }" | jq -r ".post_id")
echo POST_ID_2=$POST_ID_2
POST_ID_3=$(curl -s -X POST "localhost:3000/create_post" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"content\": \"go\" }" | jq -r ".post_id")
echo POST_ID_3=$POST_ID_3
POST_ID_4=$(curl -s -X POST "localhost:3000/create_post" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"content\": \"goooo\" }" | jq -r ".post_id")
echo POST_ID_4=$POST_ID_4

# update post
curl -s -X PUT "localhost:3000/update_post/$POST_ID_4" -H "Authorization: $TOKEN" -H "Content-Type: application/json" -d "{ \"content\": \"GGo\" }"
echo "Updated last post"

# get post
echo "Receive last post:"
curl -s "localhost:3000/get_post/$POST_ID_4" -H "Authorization: $TOKEN"
echo

# remove post
curl -s -X DELETE "localhost:3000/remove_post/$POST_ID_4" -H "Authorization: $TOKEN"
echo "Removed last post"

echo "Trying to receive last post again:"
curl -s "localhost:3000/get_post/$POST_ID_4" -H "Authorization: $TOKEN"
echo

echo "Receive all posts:"
ALL_POSTS=$(curl -s "localhost:3000/get_posts?start_id=1&count=100" -H "Authorization: $TOKEN")
echo

echo "First 2 posts:"
curl -s "localhost:3000/get_posts?start_id=1&count=2" -H "Authorization: $TOKEN"
echo
