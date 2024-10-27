# Gear Message Passing

**Gear Message Passing** is a Rust-based project that demonstrates inter-program communication within the Gear ecosystem. The project leverages actors to simulate a message-passing system where one program (actor) sends data to another, allowing the second program to perform specific actions with that data. In this example, we use `goods` and `store` modules to represent a simplified supply chain where `store` (acting as a retailer) can request items from `goods` (acting as a warehouse).

## Table of Contents

- [Overview](#overview)
- [Directory Structure](#directory-structure)
  - [Explanation of Folders](#explanation-of-folders)
- [Installation](#installation)
- [Usage](#usage)
  - [Deploying on Vara Network](#deploying-on-vara-network)
- [Project Status](#project-status)
- [Contributing](#contributing)
- [License](#license)

## Overview

This project is built to demonstrate the concept of actor-based message passing in Rust, using Gear's capabilities. Here's how it works:

- **goods**: Represents a warehouse that stores items. It acts as an actor that receives requests from the store and responds with the requested items.
- **store**: Represents a retailer that requests items from the warehouse (goods). This actor sends requests to goods and performs further actions based on the received data.

The goal is to showcase how different programs in Gear can communicate by passing messages and sharing data.

## Directory Structure

```
gear-msg-passing/
├── goods/
│   ├── io/
│   │   └── src/
│   ├── services/
│   │   └── src/
│   ├── src/
│   │   └── lib.rs
│   ├── build.rs
│   └── Cargo.toml
├── store/
│   ├── io/
│   │   └── src/
│   ├── services/
│   │   └── src/
│   ├── src/
│   │   └── lib.rs
│   ├── build.rs
│   └── Cargo.toml
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

### Explanation of Folders

- **goods**: Contains the source code for the `goods` actor, which handles requests for items and manages inventory.
  - `io`: Handles the input/output operations related to message passing.
  - `services`: Contains service modules related to the warehouse logic.
  - `src`: Contains the main code for the `goods` program, with `lib.rs` as the main entry point.
- **store**: Contains the source code for the `store` actor, which requests items from `goods`.
  - `io`: Manages input/output operations for message exchange.
  - `services`: Contains service modules related to store-specific logic.
  - `src`: Contains the main code for the `store` program, with `lib.rs` as the main entry point.

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/rockyessel/gear-msg-passing.git
   cd gear-msg-passing
   ```

2. **Build the Project**
   Build them using Cargo:

   ```bash
   cargo build --release
   ```

   This will create optimized builds for both `goods` and `store` in below:

   ```
   gear-msg-passing/
   ├── goods/...
   ├── store/...
   ├── .gitignore
   ├── target/
   │   └── wasm32-unknown-unknown/
   │       └── goods.opt.wasm
   │       └── store.opt.wasm
   │       └── goods.meta.txt
   │       └── store.meta.txt
   ├── Cargo.toml
   └── Cargo.lock
   ```

## Usage

To run this project, deploy the `goods` and `store` actors on the **Vara Network** to simulate inter-program communication in a decentralized environment. The project setup mimics a supply chain where:

- The **store** actor requests items from the **goods** actor (warehouse).
- The **goods** actor processes the request and responds with item data.

### Deploying on Vara Network

To deploy on the Vara Network, follow this [link](https://www.freecodecamp.org/news/build-and-deploy-smart-contract-rust-gear-protocol).

## Project Status

> **Note**: This project is currently **in development** and is not fully functional yet. Several key features, such as complete data handling and optimized message-passing logic, are still under construction. The example deployment on the Vara Network has been partially tested, but users may encounter issues or incomplete functionality.

Contributions and feedback are highly encouraged as we work to finalize this implementation.

## Contributing

Contributions to improve functionality, extend services, or add additional examples of actor-based interactions are welcome! Please fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
