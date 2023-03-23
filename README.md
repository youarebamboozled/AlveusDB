# AlveusDB

AlveusDB will be a database for storing and querying data from Arma Reforger Mods and probably more.

## Disclaimer

This is a work in progress and is not ready for use. I am not a database expert and this is my first attempt at creating a database. 
I am also learning Rust as I go so the code is probably not the best. I am open to suggestions and pull requests.
And I know DB isn't the correct term for this, but I couldn't think of a better one.

## Goals

* Easy interface for adding data to the database over HTTP
* Easy interface for querying data from the database over HTTP
* Good performance
* Easy to use
* Easy to install
* Easy to use for Arma Reforger mods using the built-in Web Request Module(which is broken at the moment)
* Rules for adding data to the database to prevent bad data from being added
* Readability of saved data without the need for a database viewer (e.g. JSON)

## Non-Goals

* Security (because it won't be open to the public and only accessible from the local network)
* Data encryption (because it won't be open to the public and only accessible from the local network)
* Data compression (for now at least because we don't have that much data)
* Using any HTTP framework (because I want to learn how to do it myself)

## How to use

For now, you can't. I am still working on it.

## How to install

### Building from source

#### Requirements

* Rust (nightly)
* Cargo
* Git

#### Steps

1. Clone the repository
2. Run `cargo build --release`
3. Execute the binary in `target/release/`
4. Go to `http://localhost:8080/` in your browser
5. You should see a minimalistic web page with some text
6. You are done

### Using the pre-built binaries

#### Steps

1. Download the latest release from the [releases page](#)
   1. extract the zip file if needed
2. Execute the binary
3. Go to `http://localhost:8080/` in your browser
4. You should see a minimalistic web page with some text
5. You are done

## How to contribute

Add me on Discord: `ƴøʉɐɍəɓαᵯᶋѳѳᵶᶘɇᶑ#1938` and we can talk about it.