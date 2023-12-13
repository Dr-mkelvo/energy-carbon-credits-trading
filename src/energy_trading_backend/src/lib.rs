#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use validator::Validate;

// Define type aliases for convenience
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Contract {
    password: String,
    credit_per_energy: u64,
}

// Define the structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Client {
    id: u64,
    name: String,
    phone: String,
    credits: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Producer {
    id: u64,
    name: String,
    password: String,
    phone: String,
    energy_supply: u64,
    credits: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreditOrder {
    id: u64,
    client_id: Option<u64>,
    producer_id: u64,
    credits: u64,
    min_offer_per_credit: u64,
    paid: bool,
}

// Implement the 'Storable' trait for Producer, Client and CreditOrder
impl Storable for Client {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Producer {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for CreditOrder {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Contract {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement the 'BoundedStorable' trait for the structs
impl BoundedStorable for Client {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Producer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for CreditOrder {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Contract {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define thread-local static variables for memory management and storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot inititate a counter")
    );

    static CONTRACT_STORAGE: RefCell<StableBTreeMap<u64, Contract, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static CLIENT_STORAGE: RefCell<StableBTreeMap<u64, Client, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PRODUCER_STORAGE: RefCell<StableBTreeMap<u64, Producer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static CREDIT_ORDER_STORAGE: RefCell<StableBTreeMap<u64, CreditOrder, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Define structs for payload data
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, Validate)]
struct ClientPayload {
    #[validate(length(min = 3))]
    name: String,
    #[validate(length(min = 5))]
    phone: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ProducerEnergyPayload {
    contract_password: String,
    producer_id: u64,
    energy_supply: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InitPayload {
    password: String,
    credit_per_energy: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, Validate)]
struct UpdateClientPayload {
    id: u64,
    #[validate(length(min = 3))]
    name: String,
    #[validate(length(min = 5))]
    phone: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, Validate)]
struct ProducerPayload {
    #[validate(length(min = 3))]
    name: String,
    #[validate(length(min = 5))]
    phone: String,
    #[validate(length(min = 4))]
    password: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreditOrderPayload {
    producer_id: u64,
    credits: u64,
    min_offer_per_credit: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct BidPayload {
    client_id: u64,
    credit_order_id: u64,
    offer_per_credit: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PaidPayload {
    order_id: u64,
    password: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ProducerReturn {
    id: u64,
    name: String,
    phone: String,
    energy_supply: u64,
    credits: u64,
}

// initiate the contract
#[ic_cdk::update]
fn init_contract(payload: InitPayload) -> Result<String, Error> {
    let contract = Contract {
        password: payload.password,
        credit_per_energy: payload.credit_per_energy,
    };
    match CONTRACT_STORAGE.with(|s| s.borrow_mut().insert(0, contract)) {
        Some(_) => Err(Error::InvalidPayload {
            msg: "Could not initiate contract, try again".to_string(),
        }),
        None => Ok("Contract initiated successfully".to_string()),
    }
}

// Define functions to add data to the storage
#[ic_cdk::update]
fn add_client(payload: ClientPayload) -> Result<Client, Error> {
    // Validate the payload
    let validate_payload = payload.validate();
    if validate_payload.is_err() {
        return Err(Error::InvalidPayload {
            msg: validate_payload.unwrap_err().to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    let client = Client {
        id,
        name: payload.name.clone(),
        phone: payload.phone,
        credits: 0,
    };

    match CLIENT_STORAGE.with(|s| s.borrow_mut().insert(id, client.clone())) {
        Some(_) => Err(Error::InvalidPayload {
            msg: format!("Could not add client name: {}", payload.name),
        }),
        None => Ok(client),
    }
}

// Define query functions to get client by id
#[ic_cdk::query]
fn get_client(id: u64) -> Result<Client, Error> {
    match CLIENT_STORAGE.with(|s| s.borrow().get(&id)) {
        Some(client) => Ok(client.clone()),
        None => Err(Error::NotFound {
            msg: format!("client with id: {} not found", id),
        }),
    }
}

// Define query functions to get all clients
#[ic_cdk::query]
fn get_clients() -> Result<Vec<Client>, Error> {
    // Retrieve all clients from the storage
    let clients_vec: Vec<(u64, Client)> = CLIENT_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the clients from the tuple and create a vector
    let clients: Vec<Client> = clients_vec.into_iter().map(|(_, client)| client).collect();

    // Check if any clients are found
    match clients.len() {
        0 => Err(Error::NotFound {
            msg: format!("no clients found"),
        }),
        _ => Ok(clients),
    }
}

// Define functions to update data in the storage
#[ic_cdk::update]
fn update_client(payload: UpdateClientPayload) -> Result<String, Error> {
    // get client from storage by id
    let client = CLIENT_STORAGE.with(|s| s.borrow().get(&payload.id));
    match client {
        Some(client) => {
            CLIENT_STORAGE.with(|s| {
                s.borrow_mut().insert(
                    payload.id,
                    Client {
                        name: payload.name,
                        phone: payload.phone,
                        ..client
                    },
                )
            });
            Ok(format!("Client id: {} updated successfully", payload.id))
        }
        None => Err(Error::NotFound {
            msg: "Client not found".to_string(),
        }),
    }
}

// function to add producer
#[ic_cdk::update]
fn add_producer(payload: ProducerPayload) -> Result<Producer, Error> {
    // Validate the payload
    let validate_payload = payload.validate();
    if validate_payload.is_err() {
        return Err(Error::InvalidPayload {
            msg: validate_payload.unwrap_err().to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    let producer = Producer {
        id,
        name: payload.name.clone(),
        phone: payload.phone,
        password: payload.password,
        energy_supply: 0,
        credits: 0,
    };

    match PRODUCER_STORAGE.with(|s| s.borrow_mut().insert(id, producer.clone())) {
        Some(_) => Err(Error::InvalidPayload {
            msg: format!("Could not add producer name: {}", payload.name),
        }),
        None => Ok(producer),
    }
}

// award producer carbon credits per renewable energy supply
#[ic_cdk::update]
fn award_producer_energy(payload: ProducerEnergyPayload) -> Result<String, Error> {
    // check if producer exists
    let producer = PRODUCER_STORAGE.with(|s| s.borrow().get(&payload.producer_id));
    match producer {
        Some(producer) => {
            match CONTRACT_STORAGE.with(|s| s.borrow().get(&0)) {
                Some(contract) => {
                    if contract.password != payload.contract_password {
                        return Err(Error::Unauthorized {
                            msg: "Unauthorized, method only available to contract Admins"
                                .to_string(),
                        });
                    }
                    PRODUCER_STORAGE.with(|s| {
                        s.borrow_mut().insert(
                            payload.producer_id,
                            Producer {
                                energy_supply: producer.energy_supply + payload.energy_supply,
                                credits: producer.credits
                                    + payload.energy_supply * contract.credit_per_energy,
                                ..producer.clone()
                            },
                        )
                    });
                }
                None => {
                    return Err(Error::NotFound {
                        msg: "Contract not found, please initialize contract".to_string(),
                    });
                }
            }

            Ok(format!(
                "Producer id: {} awarded successfully",
                payload.producer_id
            ))
        }
        None => Err(Error::NotFound {
            msg: "Producer not found".to_string(),
        }),
    }
}

// function to get all producers
#[ic_cdk::query]
fn get_producers() -> Result<Vec<ProducerReturn>, Error> {
    // Retrieve all producers from the storage
    let producers_vec: Vec<(u64, Producer)> =
        PRODUCER_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the producers from the tuple and create a vector
    let producers: Vec<Producer> = producers_vec
        .into_iter()
        .map(|(_, producer)| producer)
        .collect();

    // Check if any producers are found
    match producers.len() {
        0 => Err(Error::NotFound {
            msg: format!("no producers found"),
        }),
        _ => {
            let producers_return: Vec<ProducerReturn> = producers
                .into_iter()
                .map(|producer| ProducerReturn {
                    id: producer.id,
                    name: producer.name,
                    phone: producer.phone,
                    energy_supply: producer.energy_supply,
                    credits: producer.credits,
                })
                .collect();
            Ok(producers_return)
        }
    }
}

// function to get producer by id
#[ic_cdk::query]
fn get_producer(id: u64) -> Result<ProducerReturn, Error> {
    // Retrieve producer from the storage
    let producer = PRODUCER_STORAGE.with(|s| s.borrow().get(&id));

    // Check if producer is found
    match producer {
        Some(producer) => Ok(ProducerReturn {
            id: producer.id,
            name: producer.name.clone(),
            phone: producer.phone.clone(),
            energy_supply: producer.energy_supply,
            credits: producer.credits,
        }),
        None => Err(Error::NotFound {
            msg: format!("producer with id: {} not found", id),
        }),
    }
}

// function to add credit order
#[ic_cdk::update]
fn add_credit_order(payload: CreditOrderPayload) -> Result<CreditOrder, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_id = *counter.borrow().get();
            counter.borrow_mut().set(current_id + 1)
        })
        .expect("Cannot increment Ids");

    // check if producer exists and has enough credits for order
    let producer = PRODUCER_STORAGE.with(|s| s.borrow().get(&payload.producer_id));

    match producer {
        Some(producer) => {
            if producer.credits < payload.credits {
                return Err(Error::InvalidPayload {
                    msg: "Producer does not have enough credits".to_string(),
                });
            }
        }
        None => {
            return Err(Error::NotFound {
                msg: "Producer not found".to_string(),
            });
        }
    }

    let credit_order = CreditOrder {
        id,
        client_id: None,
        producer_id: payload.producer_id,
        credits: payload.credits,
        min_offer_per_credit: payload.min_offer_per_credit,
        paid: false,
    };

    match CREDIT_ORDER_STORAGE.with(|s| s.borrow_mut().insert(id, credit_order.clone())) {
        Some(_) => Err(Error::InvalidPayload {
            msg: "Invalid payload".to_string(),
        }),
        None => Ok(credit_order),
    }
}

// get all incomplete orders
#[ic_cdk::query]
fn get_all_incomplete_orders() -> Result<Vec<CreditOrder>, Error> {
    // Retrieve all credit orders from the storage
    let credit_orders_vec: Vec<(u64, CreditOrder)> =
        CREDIT_ORDER_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the credit orders from the tuple and create a vector
    let credit_orders: Vec<CreditOrder> = credit_orders_vec
        .into_iter()
        .map(|(_, credit_order)| credit_order)
        .collect();

    // Check if any credit orders are found
    match credit_orders.len() {
        0 => Err(Error::NotFound {
            msg: format!("no incomplete credit orders found"),
        }),
        _ => {
            let incomplete_orders: Vec<CreditOrder> = credit_orders
                .into_iter()
                .filter(|credit_order| !credit_order.paid)
                .collect();
            Ok(incomplete_orders)
        }
    }
}

// function to get all credit orders
#[ic_cdk::query]
fn get_all_credit_orders() -> Result<Vec<CreditOrder>, Error> {
    // Retrieve all credit orders from the storage
    let credit_orders_vec: Vec<(u64, CreditOrder)> =
        CREDIT_ORDER_STORAGE.with(|s| s.borrow().iter().collect());
    // Extract the credit orders from the tuple and create a vector
    let credit_orders: Vec<CreditOrder> = credit_orders_vec
        .into_iter()
        .map(|(_, credit_order)| credit_order)
        .collect();

    // Check if any credit orders are found
    match credit_orders.len() {
        0 => Err(Error::NotFound {
            msg: format!("no credit orders found"),
        }),
        _ => Ok(credit_orders),
    }
}

// function to get credit order by id
#[ic_cdk::query]
fn get_credit_order_by_id(id: u64) -> Result<CreditOrder, Error> {
    // Retrieve credit order from the storage
    let credit_order = CREDIT_ORDER_STORAGE.with(|s| s.borrow().get(&id));

    // Check if credit order is found
    match credit_order {
        Some(credit_order) => Ok(credit_order.clone()),
        None => Err(Error::NotFound {
            msg: format!("credit order with id: {} not found", id),
        }),
    }
}

// function for clients to bid for credit order
#[ic_cdk::update]
fn bid(payload: BidPayload) -> Result<String, Error> {
    // check if credit order exists
    let credit_order = CREDIT_ORDER_STORAGE.with(|s| s.borrow().get(&payload.credit_order_id));
    match credit_order {
        Some(credit_order) => {
            // check if client exists
            let client = CLIENT_STORAGE.with(|s| s.borrow().get(&payload.client_id));
            match client {
                Some(client) => {
                    // check if credit order has already been paid
                    if credit_order.paid {
                        return Err(Error::AlreadyPaid {
                            msg: "Credit order has already been paid".to_string(),
                        });
                    }
                    // check if client has already bid for credit order
                    if credit_order.client_id == Some(client.id) {
                        return Err(Error::InvalidPayload {
                            msg: "Client has already bid for credit order".to_string(),
                        });
                    }
                    // check if client is bidding for credit order with lower offer_per_credit
                    if credit_order.min_offer_per_credit > payload.offer_per_credit {
                        return Err(Error::InvalidPayload {
                            msg: "Client cannot bid for credit order with lower offer_per_credit"
                                .to_string(),
                        });
                    }

                    // update credit order
                    CREDIT_ORDER_STORAGE.with(|s| {
                        s.borrow_mut().insert(
                            payload.credit_order_id,
                            CreditOrder {
                                client_id: Some(payload.client_id),
                                min_offer_per_credit: payload.offer_per_credit,
                                ..credit_order.clone()
                            },
                        )
                    });
                    Ok(format!("Client id: {} bid successfully", payload.client_id))
                }
                None => Err(Error::NotFound {
                    msg: "Client not found".to_string(),
                }),
            }
        }
        None => Err(Error::NotFound {
            msg: "Credit order not found".to_string(),
        }),
    }
}

// function for producers to mark bid as paid and complete it
#[ic_cdk::update]
fn mark_order_paid(payload: PaidPayload) -> Result<String, Error> {
    // check if credit order exists
    let credit_order = CREDIT_ORDER_STORAGE.with(|s| s.borrow().get(&payload.order_id));

    match credit_order {
        Some(credit_order) => {
            let producer = PRODUCER_STORAGE.with(|s| s.borrow().get(&credit_order.producer_id));
            match producer {
                Some(producer) => {
                    if producer.password != payload.password {
                        return Err(Error::Unauthorized {
                            msg: "Unauthorized, method only available to producers".to_string(),
                        });
                    }
                }
                None => {
                    return Err(Error::NotFound {
                        msg: "Producer not found".to_string(),
                    });
                }
            }

            // check if credit order has already been paid
            if credit_order.paid {
                return Err(Error::AlreadyPaid {
                    msg: "Credit order has already been paid".to_string(),
                });
            }
            // check if client has already bid for credit order
            if credit_order.client_id.is_none() {
                return Err(Error::InvalidPayload {
                    msg: "Client has not bid for credit order".to_string(),
                });
            }

            // update producer to deduct credits
            match deduct_credit_from_producer(credit_order.producer_id, credit_order.credits) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            // update client to add credits
            match add_credit_to_client(credit_order.client_id, credit_order.credits) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
            // update credit order
            CREDIT_ORDER_STORAGE.with(|s| {
                s.borrow_mut().insert(
                    payload.order_id,
                    CreditOrder {
                        paid: true,
                        ..credit_order.clone()
                    },
                )
            });
            Ok(format!(
                "Credit order id: {} marked as paid",
                payload.order_id
            ))
        }
        None => Err(Error::NotFound {
            msg: "Credit order not found".to_string(),
        }),
    }
}

// fuction to add credit to client
fn add_credit_to_client(client_id: Option<u64>, credits: u64) -> Result<String, Error> {
    match client_id.is_some() {
        true => {
            // check if client exists
            let client = CLIENT_STORAGE.with(|s| s.borrow().get(&client_id.unwrap()));
            match client {
                Some(client) => {
                    // update client
                    CLIENT_STORAGE.with(|s| {
                        s.borrow_mut().insert(
                            client.id,
                            Client {
                                credits: client.credits + credits,
                                ..client.clone()
                            },
                        )
                    });
                    Ok(format!("Client id: {} credited successfully", client.id))
                }
                None => Err(Error::NotFound {
                    msg: "Client not found".to_string(),
                }),
            }
        }
        false => Ok("Order client not added".to_string()),
    }
}

//  fucntion to deduct credit from producer
fn deduct_credit_from_producer(producer_id: u64, credits: u64) -> Result<String, Error> {
    // check if producer exists
    let producer = PRODUCER_STORAGE.with(|s| s.borrow().get(&producer_id));
    match producer {
        Some(producer) => {
            // update producer
            PRODUCER_STORAGE.with(|s| {
                s.borrow_mut().insert(
                    producer_id,
                    Producer {
                        credits: producer.credits - credits,
                        ..producer.clone()
                    },
                )
            });
            Ok(format!(
                "Producer id: {} credited successfully",
                producer_id
            ))
        }
        None => Err(Error::NotFound {
            msg: "Producer not found".to_string(),
        }),
    }
}

// Define an Error enum for handling errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    AlreadyPaid { msg: String },
    InvalidPayload { msg: String },
    Unauthorized { msg: String },
}

// Candid generator for exporting the Candid interface
ic_cdk::export_candid!();
