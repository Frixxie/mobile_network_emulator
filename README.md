# Mobile Network Emulator

This is the practical work for my master's thesis

NB: This project requires a running MongoDB instance

The project is organized as follows: 

## Mobile Network Emulator

This is the main emulator containing the mobile network core, mobile network exposure and network.

To run
```sh
cargo r --bin mobile_network_emulator
```

## Mobile Network Orchestrator

This is the application orchestrator described in my master's thesis

To run
```sh
cargo r --bin mobile_network_orchestrator
```

## Mobile Network Core Events

This is a library crate that contains the definitions for mobile network core events
This is not an executable.

## Mobile Network Frontend

This is the frontend for the Mobile Network Emulator

To run 
```sh
cd mobile_network_frontend && npm install && npm run dev
```

## Test Client

Contains various Python programs used to run experiments

## Subscriber
Legacy folder used to test existing NEF APIs
