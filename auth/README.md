# Auth

Examples:

````bash
> curl 'localhost:3000/signup' -H 'Content-Type: application/json' -d '{ "login": "ben", "password": "big" }'
````

````bash
> curl 'localhost:3000/login' -H 'Content-Type: application/json' -d '{ "login": "ben", "password": "big" }'

{"token":"$2b$10$ES.n7JPpi6YW4vU43dSRueLueEBCsaY2EI7BL3zOuuvRsnloqETUy"}
````

````bash
> curl 'localhost:3000/update' -H 'Content-Type: application/json' -d '{ "first_name": "Ben", "last_name": "Big", "birth_date": "2012-04-23", "email": "hcenquiries@parliament.uk", "phone": "0800 112 4272", "token": "$2b$10$ES.n7JPpi6YW4vU43dSRueLueEBCsaY2EI7BL3zOuuvRsnloqETUy" }'
````