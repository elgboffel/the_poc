# Project - The POC

## Getting Started

Make sure you have rust installed on your computer.
Find docs [find](https://www.rust-lang.org/tools/install).

From to project root op the terminal and run:

`cargo install cargo-watch`

This will install 
[cargo-watch](https://crates.io/crates/cargo-watch)
so we can use hot reloading. 

## Install Yew Dependencies
Run the following commands:

`cargo install trunk`

`rustup target add wasm32-unknown-unknown`

Yew [docs](https://yew.rs/docs/tutorial).

## Install Graphql/Cynic dependencies
Run the following commands:

`cargo install --locked cynic-cli`

Run instrospect command from the crate folder.
This should already run automatically.
It is just here as an example.

`cynic introspect https://swapi-graphql.netlify.app/.netlify/functions/index -o schemas/starwars.graphql`
`

## Install Sea ORM
Install CLI

`cargo install sea-orm-cli`

Run migration from binary
`cargo run -p migration`

Generate entities
`sea-orm-cli generate entity -u postgresql://database:database@localhost:58040/database -o ./crates/entity/src --lib --with-serde both`

If you get some library errors
you probably need to install either
mysql or postgresql. On mac you can do this
with Homebrew.

`brew install postgresql`

`brew install mysql`

## Install Diesel dependencies (Not in use)
Run the following commands:

`cargo install diesel_cli`

If you get some library errors 
you probably need to install either 
mysql or postgresql. On mac you can do this 
with Homebrew.

`brew install postgresql`

`brew install mysql`
## Build and develop
Now run the following i the terminal:

`cargo watch -x "run --bin server"`

Or if you want to run the web app

`cargo watch -x "run --bin web"`

Also if you want to build you can just run:

`cargo build`

And all binaries and crates will be build.

## Run with docker compose
Run the following command from the root folder
int the terminal.

`docker-compose -f .docker/docker-compose.yml --env-file .env up --build`