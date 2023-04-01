curl -X PUT 'http://127.0.0.1:8000/users' \
     -H 'X-Api-Key: dd4a8e92-ec06-4195-a5a5-b0260f099602' \
     -H 'Content-Type: application/json' \
     --data-raw '{"email":"test","password":"test"}'