# Envwoman
Your open source `.env`-manager!


## Description
Envwoman is an application, to sync your [`.env`](https://en.wikipedia.org/wiki/Environment_variable)-files across multiple machines. The main goal is to make Envwoman secure and trustworthy, so everything is open-source and the data will never in plain-text on the server. Encryption happens client-sided via [aes-gcm](https://en.wikipedia.org/wiki/Galois/Counter_Mode).

## Installation
#### Just download
Grab the latest binary from the [Releases](https://github.com/mawoka-myblock/envwoman/releases).

#### Compile it yourself
1. `git clone https://github.com/mawoka-myblock/envwoman && cd envwoman/cli`
1. `cargo install --path ./`

## Usage
After you installed envwoman, run `envwoman login` to log in. If you don't have an account yet, head over to the [Register page](https://envwoman.mawoka.eu/register?utm_medium=Github&utm_source=Readme) and create an account there. After that, you can initialize a new project in the folder you're in, by entering `envwoman init`. If you want to add a file (probably called `.env`), use `envwoman init -f .env` instead of `envwoman init`. To push your changes, run `envwoman push`, and to pull, use `envwoman pull`. You can find out more by entering `envwoman -h`.
## Self-Hosting
1. Clone the repo: `git clone https://github.com/mawoka-myblock/envwoman && cd envwoman`
2. Build and pull the docker images: `docker compose pull && docker compose build`
3. Configure the docker-compose.yml file
4. Configure the Caddyfile-docker file
5. Run the docker-compose: `docker-compose up -d`
