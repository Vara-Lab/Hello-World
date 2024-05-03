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

9. Create a new directory named `src` in your project directory.
10. Inside the `src` directory, create a new file named `lib.rs`.
11. Add the following code to the `lib.rs` file:

    ```rust
    #![no_std]
    use gstd::{msg, prelude::*, debug};

    #[no_mangle]
    extern "C" fn handle() {
        msg::reply(String::from("Hello"), 0)
            .expect("Error in sending a reply message");
    }

    #[no_mangle]
    extern "C" fn init() {
        let init_message: String = msg::load()
            .expect("Can't load init message");
        debug!("Program was initialized with message {:?}",
            init_message);
    }
    ```

## Step 4: Compile and Deploy the Smart Contract

12. Return to your terminal and navigate to the root directory of your project.
13. Compile the smart contract by running the following command:

    ```rust
    cargo build --release
    ```

14. Once the compilation is complete, locate the `hello-world.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

## Step 5: Interact with Your Contract on Vara Network

15. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
16. Connect your Substrate wallet to Gear IDE.
17. Upload the `hello-world.opt.wasm` file by clicking the "Upload Program" button.
18. Write a message and interact with your first "Hello World" program on Vara Network.


Congratulations! You have successfully deployed your first smart contract on Vara Network. Explore further and experiment with more complex smart contracts and decentralized applications to harness the full potential of Vara Network.
