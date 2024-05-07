[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)]( https://gitpod.io/new/#https://github.com/Vara-Lab/Hello-World.git)

# Tutorial: Deploying Your First "Hello World" on Vara Network

## Introduction

Welcome to the tutorial on deploying your first "Hello World" program on Vara Network. Vara Network is a decentralized platform for deploying smart contracts and decentralized applications (dApps). This tutorial will guide you through the process of setting up your development environment and deploying a simple smart contract that prints "Hello" on the Vara Network.

## Step 1: Clone the Smart Contract Template

1. Create a GitHub account if you don't have one already.
2. Sign in to Gitpot using your GitHub account.
3. Create a new workspace on Gitpot using the following repository URL:

   ```bash
   https://github.com/Vara-Lab/Hello-World.git
   ```

## Step 2: Set Up Development Environment

4. Open your terminal and navigate to the directory where you want to store your project files.
5. Create the necessary files for your project by running the following command:

   ```bash
   touch Cargo.toml rust-toolchain.toml build.rs
   ```

6. Add the following code to the `Cargo.toml` file:

   ```rust
   [package]
   name = "hello-world"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   gstd = { git = "https://github.com/gear-tech/gear", tag = "v1.1.0" }

   [build-dependencies]
   gear-wasm-builder = { git = "https://github.com/gear-tech/gear", tag = "v1.1.0" }

   [dev-dependencies]
   gtest = { git = "https://github.com/gear-tech/gear", tag = "v1.1.0"   }
   ```

7. Add the following code to the `rust-toolchain.toml` file:

   ```rust
   [toolchain]
   channel = "nightly-2023-09-18"
   targets = ["wasm32-unknown-unknown"]
   profile = "default"
   ```

8. Add the following code to the `build.rs` file:

   ```rust
   fn main() {
       gear_wasm_builder::build();
   }
   ```

## Step 3: Implement the Smart Contract

9. Create a new directory named `src` in your project directory:

    ```bash
    mkdir src
    cd src/
    ```

10. Inside the `src` directory, create a new file named `lib.rs`:

    ```bash
    touch lib.rs
    ```

11. Add the following code to the `lib.rs` file and customize the message as needed:

    ```rust
    #![no_std]
    use gstd::{msg, prelude::*, debug};

    #[no_mangle]
    extern "C" fn handle() {
        msg::reply(String::from("Add Your Custom Message"), 0)
            .expect("Error in sending a reply message");
    }

    #[no_mangle]
    extern "C" fn init() {
        let init_message: String = msg::load()
            .expect("Can't load init message");
        debug!("Program was initialized with message: {:?}", init_message);
    }
    ```

## Step 4: Compile and Deploy the Smart Contract

12. Return to your terminal and navigate to the root directory of your project.
13. Compile the smart contract by running the following command:

    ```bash
    cargo build --release
    ```

Once the compilation is complete, locate the `hello-world.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

## Step 5: Interact with Your Contract on Vara Network

14. To interact with the IDEA gear and deploy your contract, you will need to download a wallet extension such as [Polkadot-JS](https://polkadot.js.org/extension/), [Talisman](https://talisman.xyz/), or [Subwallet](https://subwallet.app/) to interact with Substrate-based chains.

<div align="center">
  <img src="https://polkadot.js.org/extension/extension-overview.png" alt="Polkadot-JS Extension">
</div>

15. Access [Gear IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.

<div align="center">
  <img src="https://hackernoon.imgix.net/images/77WjQmBCAIQ7dyhZ22Bkui5QTrb2-6n92fqm.jpeg" alt="Gear Protocol">
</div>

16. Connect your Substrate wallet to Gear IDEA.
17. Upload the `hello-world.opt.wasm` file by clicking the "Upload Program" button.
18. Write a message and interact with your first "Hello World" program on Vara Network.
19. You will need a Hex to ASCII Converter to properly format your messages. You can use this online [Hex to ASCII Converter](https://www.rapidtables.com/convert/number/hex-to-ascii.html).

Congratulations! You have successfully deployed your first smart contract on Vara Network. Explore further and experiment with more complex smart contracts and decentralized applications to harness the full potential of Vara Network.

````
