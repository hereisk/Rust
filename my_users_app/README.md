# Welcome to My Users App
***

## Task
My Users App is an implementation of the MVC (Model View Controller) architecture.

## Description
There are 3 components of the program
1. Model (database) is based on sqlite3. All interections with the database are done through struct User
2. Controller is based on the rocket crate.
3. View implimentation is rudementary and is only supported for one request.

## Installation
To run the server run the command "cargo run" from within my_users_app folder.
The server will run at http://127.0.0.1 with port 8000.

## Usage
There are the examples of the supported routs and commands

curl -X POST http://127.0.0.1:8000/users -d "firstname=Jordan" -d "lastname=Richards" -d "age=22" -d "password=jr22" -d "email=jordan@richards.com"
• will create a user

curl -X GET http://127.0.0.1:8000/users
• will return the list of all users in the JSON format

curl -c cookie.txt -X POST http://127.0.0.1:8000/sign_in -d "password=jr22" -d "email=jordan@rich.com"
• will log in the user into the database. The relevant information is being saved to cookie.txt for future user

curl -b cookie.txt -X PUT http://127.0.0.1:8000/sign_in -d "password=rj22"
• will update the password for a logged in user

curl -b cookie.txt -X DELETE http://127.0.0.1:8000/sign_out
• will sign the user out. The current version of the program does not remove the cookie.txt file.

curl http://127.0.0.1:8000/
• will return an html with the table with all users in the database

### The Core Team
The project is completed by Konstantin Melkov