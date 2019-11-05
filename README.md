# Onion or News?

A simple guessing game that scrapes reddit's
[r/TheOnion](https://reddit.com/r/TheOnion) and
[r/NotTheOnion](https://reddit.com/r/nottheonion).
Players have to guess whether the given article title
comes from the satirical publication The Onion
or is actual news.

[Live demo](https://onionornews.com)

# Running

## Prerequisites
* PostgreSQL
* Rust 1.39 or higher

A docker-compose.yml file is provided for development usage.
Executables automatically apply schema migrations.

## Example

    $ docker-compose up -d
    $ export DATABASE_URL='postgres://oon:hunter2@localhost/oon'
    
    $ # Collect articles from reddit
    $ cd oon_scraper && cargo run
    
    $ # Run, optionally specifying a port and the log level
    $ cd oon_web && PORT=1234 RUST_LOG=info cargo run

# Architecture

* `oon_db` is a database layer that uses Diesel and R2D2 pooling.
  Articles ("questions") and player results ("answers") are stored.
* `oon_scraper` walks reddit's public API
  to collect article titles and categories (onion / news).
* `oon_web` contains an HTTP server using Warp
  that provides a JSON endpoint for submitting answers
  and a bare bones front-end using Vue.js.
