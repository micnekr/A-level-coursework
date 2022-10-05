# Computer science coursework

## Setup
1. Create self-signed temporary certificate for TLS in the ssl folder
`cd server/ssl/`
`openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
2. Set up a .env file
`cp .env.example .env`
Replace the <placeholders> with names of your choice
3. Generate a diesel database
`diesel database reset`
