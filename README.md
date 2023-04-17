# Ethereum Uniswap Event Listener

This is a simple Rust program that listens for Swap events on the WETH/USDC Uniswap smart contract (V3) on the Ethereum Mainnet. The program connects to the Ethereum network via Infura's WebSocket API and prints relevant information about each swap event to the console.

## Features

- Listens for new Swap events on the WETH/USDC Uniswap smart contract (V3) on the Ethereum Mainnet.
- Prints the following information for each swap event to the console:
  - Timestamp
  - Block number
  - Transaction hash
  - Sender address
  - Recipient address
  - Amount of USDC
  - Amount of WETH

## Prerequisites

- Rust installed on your system.
- An Infura account with a valid API key.

## Usage

1. Clone this repository:

    ```sh
    git clone https://github.com/ahmadkaouk/uniswap-listener.git
    cd uniswap-listener
    ```

2. Compile the Rust program:

    ```sh
    cargo build --release
    ```

3. Run the program with your Infura API key as a command-line argument:

    ```sh
    ./target/release/ethereum-uniswap-event-listener <infura_api_key>
    ```

    Replace <infura_api_key> with your actual Infura API key.

4. The program will start listening for new Swap events on the WETH/USDC Uniswap smart contract and print the relevant information to the console.
