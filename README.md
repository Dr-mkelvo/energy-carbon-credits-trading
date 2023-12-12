# Energy Credit Management System

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
- [Data Structures](#data-structures)
- [Memory Management](#memory-management)
- [Functions](#functions)
- [Error Handling](#error-handling)

## Overview

This is a Renewable Energy Credit Management System is designed to facilitate the exchange of carbon credits between electricity producers and clients. Producers earn credits proportional to their renewable energy supply, and  Clients, on the other hand, can bid on credit orders, creating a dynamic marketplace for carbon credits. The system also enables the initiation of contracts, addition of clients and producers, and management of credit orders.

## Usage

To use the system, clone the github repository to your computer.
Run the commands below to run the program.

```bash
    git clone https://github.com/your_repo/your_program.git
    cd your_program
```

## Data Structures

### Contract

- Represents the contract details, including a password and the credit-per-energy value.

### Client

- Represents a client with an ID, name, phone number, and available credits.

### Producer

- Represents a producer with an ID, name, password, phone number, energy supply, and available credits.

### CreditOrder

- Represents a credit order with an ID, associated client and producer IDs, credits, minimum offer per credit, and a paid status.

## Memory Management

Memory is allocated using a `MemoryManager` from the `ic-stable-structures` crate:

```rust
static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = // initialized
```

This manages allocating `VirtualMemory` for storages.

## ID Generation

Unique IDs are generated using a thread-local `IdCell`:

```rust
static ID_COUNTER: RefCell<IdCell> = // initialized
```

The counter is incremented when adding new records.

## Record Storage

Records are stored in thread-local `StableBTreeMap`s:

The system utilizes thread-local static variables for memory management and storage. These include:

- **MEMORY_MANAGER**: Manages virtual memory.
- **ID_COUNTER**: Manages unique IDs.
- **CONTRACT_STORAGE**: Stores contracts.
- **CLIENT_STORAGE**: Stores clients.
- **PRODUCER_STORAGE**: Stores producers.
- **CREDIT_ORDER_STORAGE**: Stores credit orders.

```rust
static CLIENT_STORAGE: RefCell<StableBTreeMap<u64, Client>> = // initialized
```

## Traits

The `Storable` and `BoundedStorable` traits are implemented for serialization and bounding record sizes during storage.

## Payloads

Payload struct for initiating the contract, Client, Producer, Credit order, bidding data, update client and payload to mark bid as paid. They carry the neccesary data for each field as needed by the functions.

### ProducerReturn

Struct for returning simplified producer information, including ID, name, phone number, energy supply, and available credits.

## Functions

### `init_contract(payload: InitPayload) -> Result<String, Error>`

Initiates the contract by setting the contract password and credit-per-energy value.

### `add_client(payload: ClientPayload) -> Result<Client, Error>`

Adds a new client to the system, including a name and phone number.

### `get_client(id: u64) -> Result<Client, Error>`

Retrieves a client by their unique ID.

### `get_clients() -> Result<Vec<Client>, Error>`

Retrieves a list of all clients in the system.

### `update_client(payload: UpdateClientPayload) -> Result<String, Error>`

Updates client information, including name and phone number.

### `add_producer(payload: ProducerPayload) -> Result<Producer, Error>`

Adds a new electricity producer to the system, including name, phone number, and password.

### `award_producer_energy(payload: ProducerEnergyPayload) -> Result<String, Error>`

Awards energy to a producer based on the contract specifications.

### `get_producers() -> Result<Vec<ProducerReturn>, Error>`

Retrieves a list of all electricity producers with simplified information.

### `get_producer(id: u64) -> Result<ProducerReturn, Error>`

Retrieves detailed information about a specific electricity producer.

### `add_credit_order(payload: CreditOrderPayload) -> Result<CreditOrder, Error>`

Adds a new credit order to the system, specifying the producer, credits, and minimum offer per credit.

### `get_all_incomplete_orders() -> Result<Vec<CreditOrder>, Error>`

Retrieves all incomplete credit orders in the system.

### `get_all_credit_orders() -> Result<Vec<CreditOrder>, Error>`

Retrieves a list of all credit orders in the system.

### `get_credit_order_by_id(id: u64) -> Result<CreditOrder, Error>`

Retrieves detailed information about a specific credit order.

### `bid(payload: BidPayload) -> Result<String, Error>`

Allows clients to bid on a specific credit order.

### `mark_order_paid(payload: PaidPayload) -> Result<String, Error>`

Allows producers to mark a credit order as paid.

## Error Handling

The system uses the `Error` enum for handling various error scenarios, including not found, already paid, invalid payload, and unauthorized actions.

## More

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with energy_trading, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.icp0.io)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd energy_trading/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `production` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
