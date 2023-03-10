# CosmWasm Oracle

This is a data oracle contract built on the CosmWasm smart contract platform. It is designed with generalization in mind, allowing for a developer to submit and data to the contract for their needed use case.

To submit data, you just have to modify the data-feeder application with the specific endpoints and logic for each individual to run. For the best outcome, you should take the average if you query multiple sources.

Example use cases:

- [Crypto Token Prices (Typescript)](./data-feeders-examples/crypto-prices/)
- [Stock prices (Typescript)](./data-feeders-examples/stock-prices/)
- Sports Game points
- Weather

All of these are possible with this oracle contract.
