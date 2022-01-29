# Envwoman
Your open source `.env`-manager!


## Description
Envwoman is an application, to sync your [`.env`](https://en.wikipedia.org/wiki/Environment_variable)-files across multiple machines. The main goal is to make Envwoman secure and trustworthy, so everything is open-source and the data will never in plain-text on the server. Encryption happens client-sided via [fernet](https://github.com/fernet/spec/).



## Self-Hosting
1. Clone the repo: `git clone https://github.com/mawoka-myblock/envwoman && cd envwoman`
2. Build and pull the docker images: `docker compose pull && docker compose build`
3. Configure the docker-compose.yml file
4. Configure the Caddyfile-docker file
5. Run the docker-compose: `docker-compose up -d`
