[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)]( https://gitpod.io/new/#https://github.com/Vara-Lab/Hello-World.git)

# Tutorial: Deploying Your First "Hello World" on Vara Network

## Introduction

Welcome to the tutorial on deploying your first "Hello World" program on Vara Network. Vara Network is a decentralized platform for deploying smart contracts and decentralized applications (dApps). This tutorial will guide you through the process of setting up your development environment and deploying a simple smart contract that prints "Hello" on the Vara Network.

## Step 1: Clone the Smart Contract Template

1. Create a GitHub account if you don't have one already.
2. Sign in to Gitpod using your GitHub account.
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
  <img src="https://hackernoon.imgix.net/images/77WjQmBCAIQ7dyhZ22Bkui5QTrb2-6n92fqm.jpeg" alt="Gear Protocol">
</div>


## Step 6: Upload Your Contract on Vara Network

18. Upload the `hello-world.opt.wasm` file by clicking the "Upload Program" button.
<div align="center">
  <img src="data:image/png;base64,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" alt="Gear Protocol">
</div>

19. Specify the program Name and click the Calculate Gas button. The Gas limit will be set automatically. Now click the Upload program button.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/interface-f325a652ff7a91fa44bfa84c8b482757.png" alt="Gear Protocol Interface">
</div>

20. Sign the program uploading the transaction to the Gear network. After your message has been successfully processed, you are to see correspondent log messages.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/message-log-158efeb8c52fca9fcc080c40561c36df.png" alt="Signing Transaction">
</div>

21. Once your program is uploaded, head to the Programs section and find your program.

<div align="center">
  <!-- Considera cambiar esta imagen si no representa el paso de encontrar tu programa en la sección de programas -->
  <img src="https://wiki.gear-tech.io/assets/images/sign-transaction-f9ae773fdad49788a0e9894238ba5558.png" alt="Find Your Program">
</div>

## Step 7: Send a Message to a Program

22. Now, try sending your newly uploaded program a message to see how it responds! Click the "Send message" button.

23. In the Payload field of the opened dialog, write a message. Then click the "Calculate Gas" button; the Gas limit will be set automatically. Now, click the "Send Message" button to interact with your first "Hello World" program on Vara Network.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/send-request-2b80a05597793a6c7c1c07e72578ccbe.png" alt="Sending Message Interface">
</div>

24. Sign the message sending transaction. To properly format your messages, you will need a Hex to ASCII Converter. You can use this online [Hex to ASCII Converter](https://www.rapidtables.com/convert/number/hex-to-ascii.html).

Congratulations! You have successfully deployed your first smart contract on Vara Network. Explore further and experiment with more complex smart contracts and decentralized applications to harness the full potential of Vara Network.


````
