[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)]( https://gitpod.io/new/#https://github.com/Vara-Lab/Hello-World.git)

# Tutorial: Deploying Your First "Hello World" on Vara Network

## Introduction

Welcome to the tutorial on deploying your first "Hello World" program on Vara Network. Vara Network is a decentralized platform for deploying smart contracts and decentralized applications (dApps). This tutorial will guide you through the process of setting up your development environment and deploying a simple smart contract that prints "Hello" on the Vara Network.

## Step 1: Clone the Smart Contract Template

1. Create a GitHub account if you don't have one already.
2. Sign in to Gitpod using your GitHub account.[![Open in Gitpod]]( https://gitpod.io/new/#https://github.com/Vara-Lab/Hello-World.git)
3. Create a new workspace on Gitpod using the following repository URL:

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

16. Connect your Substrate wallet to Gear IDEA by clicking the connect button.
17. Navigate to [Gear IDEA](https://idea.gear-tech.io). You will be prompted to grant access to your account for the Gear Tech application. Click "Yes, allow this application access".

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/polkadot-access-c98e0c0e2df8de4cb5673f80e81743ac.png" alt="Gear Protocol">
</div>


## Step 6: Upload Your Contract on Vara Network

18. Upload the `hello-world.opt.wasm` file by clicking the "Upload Program" button.

19. Specify the program Name and click the Calculate Gas button. The Gas limit will be set automatically. Now click the Upload program button.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/interface-f325a652ff7a91fa44bfa84c8b482757.png" alt="Gear Protocol Interface">
</div>

20. Sign the program uploading the transaction to the Gear network. After your message has been successfully processed, you are to see correspondent log messages.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/sign-transaction-f9ae773fdad49788a0e9894238ba5558.png" alt="Find Your Program">
</div>

21. Once your program is uploaded, head to the Programs section and find your program.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/message-log-158efeb8c52fca9fcc080c40561c36df.png" alt="Signing Transaction">
</div>

## Step 7: Send a Message to a Program

22. Now, try sending your newly uploaded program a message to see how it responds! Click the "Send message" button.

23. In the Payload field of the opened dialog, write a message. Then click the "Calculate Gas" button; the Gas limit will be set automatically. Now, click the "Send Message" button to interact with your first "Hello World" program on Vara Network.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/send-request-2b80a05597793a6c7c1c07e72578ccbe.png" alt="Sending Message Interface">
</div>

24. Sign the message sending transaction. To properly format your messages, you will need a Hex to ASCII Converter. You can use this online [Hex to ASCII Converter](https://www.rapidtables.com/convert/number/hex-to-ascii.html).

Congratulations! You have successfully deployed your first smart contract on Vara Network. Explore further and experiment with more complex smart contracts and decentralized applications to harness the full potential of Vara Network.

