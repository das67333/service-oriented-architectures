response_code=$(curl -s -o /dev/null -w "%{http_code}" -X POST $HOST/signup -H "$SJ" -d "{ \"login\": \"$LOGIN\", \"password\": \"$PASSWORD\" }")

if [ "$response_code" -eq 200 ]; then
  echo "Test passed: Return code is 200"
else
  echo "Test failed: Return code is not 200"
fi