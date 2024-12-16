# Car Rental & Tokenization Platforms

This project is a decentralized application (dApp) for car rental management and tokenization, leveraging Internet Computer (ICP) canisters for backend functionality. It includes:

1. **Car Rental Platform** for managing car rentals and bookings.
2. **Car Tokenization Platform** for tokenizing cars as NFTs (Non-Fungible Tokens).

## Canisters Overview

### 1. Car Rental Platform Canister

| **Name**   | **Package** | **Type** | **Description**                      |
|------------|-------------|----------|--------------------------------------|
| **backend** | `backend`  | Rust     | Manages cars and booking operations. |

### 2. Car Tokenization Platform Canisters

| **Name**       | **Package**     | **Type**    | **Description**                                                        |
|----------------|-----------------|-------------|------------------------------------------------------------------------|
| **provision**  | `provision`     | Rust        | Acts as an index canister and provisions new NFT canisters.            |
| **asset_proxy**| `asset_proxy`   | Rust        | Acts as an index canister and provisions asset storage canisters.      |
| **asset**      | Custom          | Wasm Module | Stores assets temporarily for token canisters.                         |
| **token**      | `token`         | Rust        | NFT minter canister, manages NFT collections, minting, and sales.      |

# Architecture Diagram

```mermaid
flowchart TD
    %% Car Rental Platform
    subgraph Car_Rental_Platform["Car Rental Platform"]
        CarRentalFrontend["Car Rental frontend"]
        BackendCanister["Backend Canister<br>Cars and booking<br>Management"]
        RentalOffchain["Rental Offchain<br>Service<br>Sends booking<br>confirmation email"]

        CarRentalFrontend --> BackendCanister
        CarRentalFrontend --> RentalOffchain
        RentalOffchain --> BackendCanister
    end

    %% Car Tokenization Platform
    subgraph Car_Tokenization_Platform["Car Tokenization Platform"]
        CarTokenizationFrontend["Car tokenization frontend"]
        ProvisionCanister["Provision Canister<br>Acts as an index<br>canister and provisions<br>new NFT canisters"]
        AssetProxyCanister["Asset Proxy Canister<br>Acts as an index<br>canister and provisions<br>asset storage canisters"]
        AssetCanister["Asset Canister<br>Acts as a temporary<br>asset storage"]

        CarTokenizationFrontend --> ProvisionCanister
        ProvisionCanister --> AssetProxyCanister
        AssetProxyCanister --> AssetCanister

        subgraph Token_Storage["Token Storage"]
            TokenCanister1["Token canister<br>NFT minter canister<br>Acts as an NFT collection<br>Used to mint new NFTs<br>And manage sales"]
            AssetCanister1["Asset canister<br>Acts as an NFT storage<br>for each NFT minter canister"]

            TokenCanister2["Token canister<br>NFT minter canister<br>Acts as an NFT collection<br>Used to mint new NFTs<br>And manage sales"]
            AssetCanister2["Asset canister<br>Acts as an NFT storage<br>for each NFT minter canister"]

            TokenCanister1 --> AssetCanister1
            TokenCanister2 --> AssetCanister2
        end

        ProvisionCanister --> TokenCanister1
        ProvisionCanister --> TokenCanister2
    end

    %% Frontend Clients
    FrontendClients["Frontend clients -<br>Web, Mobile, PWA"]

    CarRentalFrontend --> FrontendClients
    CarTokenizationFrontend --> FrontendClients
```

# Deploying Canisters

## Local Deployment

To deploy the canisters locally using the DFX (Dfinity SDK), run the following script:

```bash
dfx start --background
./scripts/local_deploy.sh
```