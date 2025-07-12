# VolatileGuard: Decentralized Crypto Price Alert System

A Rust-based system delivering real-time crypto price alerts triggered by smart contract events and on-chain data analysis via a decentralized WebSocket API.

VolatileGuard provides a robust and decentralized solution for monitoring cryptocurrency prices and receiving alerts based on complex, user-defined triggers. Unlike centralized price alert services, VolatileGuard leverages on-chain data analysis and smart contract event monitoring to offer highly customizable and reliable notifications. This allows users to define alerts based not only on simple price thresholds but also on intricate conditions derived from on-chain activities, such as large token transfers, liquidity pool movements, or smart contract state changes. The system is built with a focus on decentralization, ensuring that alerts are triggered and delivered independently of any single point of failure, increasing its resilience and trustworthiness.

The core of VolatileGuard is a Rust-based backend that efficiently processes on-chain data, monitors smart contract events, and evaluates user-defined trigger conditions. When a trigger is activated, the system pushes a notification to the user's client via a decentralized WebSocket API. This API is designed to be easily integrated with various applications, including trading bots, portfolio management tools, and mobile alert systems. By utilizing WebSockets, VolatileGuard ensures that alerts are delivered in real-time, enabling users to react quickly to market changes. Furthermore, the system is designed to be highly scalable, allowing it to handle a large number of users and triggers without compromising performance.

VolatileGuard aims to empower users with more control over their crypto monitoring by providing a flexible and customizable alert system. It moves beyond basic price tracking to offer advanced on-chain analysis capabilities, allowing users to stay ahead of market movements and make informed decisions. The decentralized nature of the system enhances its reliability and trustworthiness, making it a valuable tool for both individual traders and institutional investors.

## Key Features

*   **Smart Contract Event Monitoring:** Real-time monitoring of specified smart contract events (e.g., `Transfer`, `Deposit`, `Withdrawal`) using a Rust-based event listener. The event listener subscribes to the relevant logs on the blockchain and parses the event data according to the smart contract ABI.
*   **On-Chain Data Analysis:** Utilizes a dedicated module written in Rust to perform real-time analysis of on-chain data, including token balances, liquidity pool ratios, and transaction volumes. This allows for the creation of triggers based on complex conditions derived from blockchain data.
*   **User-Defined Triggers:** Allows users to define custom trigger conditions using a domain-specific language (DSL) implemented in Rust. The DSL supports mathematical expressions, logical operators, and access to on-chain data and smart contract event parameters.
*   **Decentralized WebSocket API:** Provides a decentralized WebSocket API for receiving real-time price alerts. The API leverages a peer-to-peer network to ensure that alerts are delivered reliably and without censorship.
*   **Rust Implementation for Performance:** The entire backend is implemented in Rust, providing high performance, memory safety, and concurrency, ensuring that alerts are delivered quickly and reliably even under heavy load.
*   **Scalable Architecture:** Designed with a microservices architecture to allow for horizontal scaling. Individual components can be scaled independently to meet changing demand.
*   **Configuration Management:** Uses environment variables and configuration files for easy customization and deployment.

## Technology Stack

*   **Rust:** The primary programming language for the backend, providing performance, safety, and concurrency.
*   **Web3.rs:** Rust library for interacting with the Ethereum blockchain. Used for subscribing to smart contract events and retrieving on-chain data.
*   **WebSocket:** A communication protocol providing full-duplex communication channels over a single TCP connection, enabling real-time delivery of alerts.
*   **Tokio:** An asynchronous runtime for Rust, enabling efficient handling of concurrent tasks and network operations.
*   **IPFS:** Used for storing and distributing configuration files and smart contract ABIs in a decentralized manner.

## Installation

1.  **Install Rust:** Ensure you have Rust installed. You can download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Clone the Repository:**
    git clone https://github.com/uhsr/VolatileGuard.git
    cd VolatileGuard
3.  **Install Dependencies:**
    cargo build --release
4.  **Copy the Example Configuration:** Copy the 'example.env' file to '.env'

## Configuration

VolatileGuard relies on environment variables for configuration. The following variables must be set:

*   `ETHEREUM_NODE_URL`: The URL of your Ethereum node (e.g., Infura, Alchemy).
*   `WEBSOCKET_PORT`: The port on which the WebSocket server will listen.
*   `CONTRACT_ADDRESS`: The address of the smart contract to monitor.
*   `IPFS_GATEWAY_URL`: The URL of the IPFS gateway to use for retrieving ABIs.
*   `LOG_LEVEL`: The logging level (e.g., `info`, `debug`, `error`).

Example '.env' file content:
ETHEREUM_NODE_URL=https://mainnet.infura.io/v3/<YOUR_INFURA_PROJECT_ID>
WEBSOCKET_PORT=8080
CONTRACT_ADDRESS=0x...
IPFS_GATEWAY_URL=https://ipfs.io/ipfs/
LOG_LEVEL=info

## Usage

1.  **Run the Backend:**
    cargo run --release
2.  **Connect to the WebSocket API:** You can connect to the WebSocket API using any WebSocket client. The endpoint is `ws://<server_address>:<WEBSOCKET_PORT>`. After connection, you need to send a message to register the specific triggers you're interested in. The message should be a JSON string containing the contract address and the desired events. Example trigger: '{"contract": "0x...", "event": "Transfer", "condition": "value > 1000"}'
3.  **Receive Alerts:** Once a trigger condition is met, the backend will push a message to your WebSocket client with the details of the event.

## Contributing

We welcome contributions to VolatileGuard! Please follow these guidelines:

*   Fork the repository and create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Submit a pull request with a detailed description of your changes.
*   Ensure that all tests pass before submitting your pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/uhsr/VolatileGuard/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the Rust community and the open-source contributors who have helped make this project possible.