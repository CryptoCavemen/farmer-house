# Farmer House - An In-depth Guide to Using Fusion: NFT Composability made Easier with FarmerHouse
## Author: [Gabriel Warmling](https://github.com/bielwarm)

In the world of Non-Fungible Tokens (NFTs), composability is a game-changer. Imagine an NFT akin to a backpack, capable of holding various item NFTs inside it, or a farm field NFT with slots for different plant NFTs to grow in it. As dynamic and intriguing as it sounds, realizing this isn't an easy feat, and that's where **Fusion** enters the scene.

Fusion is an NFT composability feature brought to life by the Trifle Program, a sophisticated mechanism built upon the Escrow extension of Token Metadata. It introduces **Creator Owned Escrow** (COE) and uses a **Trifle PDA** (Program Derived Address) as the creator and manager of the COE. Fusion's main objective is to bring on-chain tracking and composability around NFT ownership to the fore, thereby allowing for intricate ownership models to be implemented by creators.

Two critical components make Fusion function seamlessly: the **Escrow Constraint Model** and the **Trifle** account. The Constraint Model represents a set of restrictions and requirements, which allow for the transmission of tokens into and out from the Trifle account. On the other hand, the Trifle account is instrumental in tracking tokens owned by the COE on-chain, and manages tokens following the slot semantics of the Constraint Model.

This article dives deeper into Fusion through its use within an interactive game environment, embodied in a program/smart contract called **FarmerHouse**. Specifically designed to work in a game format, FarmerHouse creates an engaging way of demonstrating Fusion's utility. Its role is to provide an intuitive interface to visualize and interact with NFT composability, making the abstract concept more accessible to users.

Whether you're a novice eager to venture into the realm of NFTs or a seasoned player, this comprehensive guide will navigate you through Fusion's features, making NFT composability a smooth and enriching experience. So, let's embark on this journey!

## Farmer House Program: An Overview

The Farmer House program is an interactive farming simulation game that showcases Fusion, an advanced composability feature for Non-Fungible Tokens (NFTs) brought to life by the Trifle Program. Fusion's main objective is to allow for intricate ownership models to be implemented, enabling on-chain tracking and composability of NFT ownership.

### The Concept

The Farmer House program places users in the role of virtual farmers. Their journey begins with owning a unique Field NFT, symbolizing their personal farm within the game. The Field NFT is not just a representation of their farm; it's connected to a Trifle Account, set up to hold other NFTs, akin to a field ready to accommodate various crops.

Users then proceed to purchase, plant, nurture, and harvest various Crop NFTs (like Tomato NFTs) within their fields. Each Crop NFT goes through different growth stages, transitioning from a seed to a sapling and finally into a ripe crop. The progression is reflected by changes in the NFT metadata managed by the Farmer House program's Program Derived Address (PDA).

When the Crop NFTs mature, users can harvest them and sell them back to the shop, potentially making a profit from their virtual farming journey.

### The Workflow

The Farmer House workflow encapsulates the following stages:

1. **Field Purchase & Trifle Account Creation**: Users initiate their farming journey by purchasing a Field NFT, which represents their personal farm. Alongside, a Trifle Account, designed to hold Crop NFTs, is also set up and connected to the Field NFT.

2. **Seed Purchase**: Users buy Crop NFTs, for example, Tomato NFTs, which are effectively seeds ready to be planted within their fields.

3. **Planting**: Users plant their Tomato NFTs in their Field NFT. The Fusion's Trifle Program facilitates this by transferring the Crop NFTs into the Field NFT, marking the onset of the growth process.

4. **Growth Stages**: As time progresses, users nurture their crops. This care is mirrored in the Tomato NFT's metadata, signifying the growth from a seed, to a sapling, and eventually to a ripe tomato.

5. **Harvesting**: When the crops are ripe, users harvest them by transferring the ripe Tomato NFT from the Field NFT back to the user's wallet, signifying a successful harvest. The NFT transfer is facilitated by Fusion's Trifle Program.

6. **Selling**: The last step in the game workflow is selling. Users can sell their harvested ripe Tomato NFTs back to the shop. The mature Tomato NFTs fetch a higher selling price, potentially earning users a profit.

![image](https://github.com/bielwarm/farmer-house/assets/97843437/99f9b349-e5b0-4b5c-86c6-65a6d6c55be8)


Through Farmer House, Fusion's utility is demonstrated in an engaging game format. The program provides an intuitive interface to visualize and interact with NFT composability, making the abstract concept accessible to users. As a user, whether you're new to the realm of NFTs or a seasoned player, Farmer House offers a comprehensive guide to Fusion's features, simplifying NFT composability into a smooth and enriching experience. So, let's embark on this journey together!


## Step 1: Preparation - Setting up the Required Accounts, Addresses, and Tokens

Before diving into the practical use of FarmerHouse, it's crucial to set the stage with the right components. Let's familiarize ourselves with the essential accounts, addresses, and tokens needed for a smooth run of the FarmerHouse program.

1. **PAW (Program Admin Wallet)**: This wallet is used for setting up the program's Program Derived Address (PDA) and the escrow constraint model. While the PAW is used as a signer during the setup phase of the program, its usage is limited to occasional verification of the program's PDA during actual program operation.

2. **UserWallet**: The primary holder of the NFTs, this wallet belongs to the user.

3. **CurrencyToken**: This token serves as the primary currency for the program, facilitating the purchase and sale of farm fields, seeds, and plants.

4. **FieldNFT**: This NFT represents the farming field equipped with slots for sowing seeds and growing plants.

5. **FieldCollectionNFT**: This NFT acts as a safety measure, serving as the collection for fields that are verified on-chain.

6. **TomatoNFT**: Representing the plant we're farming, the TomatoNFT is our chief crop.

7. **TomatoSeedCollectionNFT, TomatoSaplingCollectionNFT, RipeTomatoCollectionNFT**: These NFTs serve as collections for our TomatoNFT, signifying the growth stages of our Tomato plant.

8. **FarmsPdaAddress**: Derived from the prefix "farmer-house-farms" and the FarmerHouse programId, this address holds critical information such as all collection addresses, the CurrencyToken mint address, and the PDA Authority address (PAW wallet address). It also functions as a "shop" for farm fields and seeds/plants. This PDA will be the authority for the Trifle account that we will create later, ensuring any interaction with the Trifle account is safely authorized by the FarmerHouse program.

These are the key components that constitute the backbone of the FarmerHouse program. Throughout the following steps, we will be creating additional accounts and addresses as needed to fully explore and utilize the functionalities of the program. Stay tuned!

> **Note:** The PAW and UserWallet should be in your control and secured properly. Always remember to take necessary precautions when dealing with wallets and keys.

In the next section, we'll guide you through the process of initializing these accounts and setting up the FarmerHouse program for use.


## Step 2: Initialization - Getting FarmerHouse Up and Running

With all the necessary accounts in place, we now proceed to initialize FarmerHouse. This involves several steps including funding the wallets, minting the required tokens, collections and NFTs, initializing our FarmsPDA, and creating the Escrow Constraint Model. Before we delve into the specifics, ensure both your **PAW** and **UserWallet** are adequately funded with SOL to facilitate account initialization and blockchain transactions.

> **Tip:** If you are working in a local or development environment, consider funding these wallets through airdrops. However, for the MainNet, you need real SOL in your wallets.

### Minting Tokens and NFTs

As FarmerHouse doesn't handle minting functionality, you'll need to mint the tokens and NFTs independently. This is because we aim to keep our focus on Fusion, and also, there are many programs, like the metaplex-foundation, that provide various ways to mint tokens and NFTs in your preferred language. However, we'll briefly cover what needs to be done:

- **CurrencyToken Minting:** Create an SPL token with at least 6 decimals, as this will serve as the program's primary currency. Mint at least 100 tokens and make sure the Associated Token Account (ATA) of the UserWallet for this mint holds at least 72 (or 72,000,000 due to decimal settings) tokens. It's best practice to mint to the ATA of PAW first and then transfer tokens to the UserWallet's ATA. Remember to store the currencyMintAddress of the currencyToken for future use.

- **NFT Collection Minting:** Mint four NFT collections, namely: FieldCollectionNFT, TomatoSeedCollectionNFT, TomatoSaplingCollectionNFT, and RipeTomatoCollectionNFT. Mint these to PAW and save the mintAddress and the metadataAddress for each.

### Initializing FarmsPDA

Having completed the minting, we now focus on initializing FarmsPDA. To do this, we'll interact with FarmerHouse, and for every new method we introduce in this tutorial, we'll explain each account involved in the method call.

First, derive the FarmsPDA address using the "findProgramAddressSync" function from the solana or anchor SDK. The seeds for this function are the byte-string "farmer-house-farms" and the FarmerHouse programId as a Buffer. Here's what it looks like in JavaScript:

```javascript
const [farmsPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode('farmer-house-farms'),
    FARMER_HOUSE_PROGRAM_ID.toBuffer(),
  ],
  FARMER_HOUSE_PROGRAM_ID
);
```

Next, derive the Associated Token Account (ATA) of the FarmsPDA with the currencyMintAddress. With all the necessary accounts and addresses in place, you're now ready to call the initializeFarmsPda method.

For this method, you'll need to provide the following arguments:

- `tomatoSeedCollectionMintAddress`
- `tomatoSapplingCollectionMintAddress`
- `ripeTomatoCollectionMintAddress`
- `fieldCollectionMintAddress`

Additionally, the following accounts will need to be provided:

- `pdaAuthority` (Signer and mutable) - The PAW, which will be registered as the authority for the farmsPda.
- `farmsPda` (Init) - The FarmsPDA, which will store all the information previously described.
- `pdaAssociatedTokenAccount` (Mutable) - The ATA for the FarmsPda and SplMint.
- `splMint` - The mint address of the CurrencyToken.
- `farmerHouseProgram` - The programId of FarmerHouse.

> **Note:** Remember that the "pdaAuthority" needs to be a signer. After completing these steps, your FarmerHouse program is set and ready for use!

### Transferring CurrencyTokens to the FarmsPDA

The next task is to equip the FarmsPDA with the necessary CurrencyTokens. These tokens will facilitate the functionalities within the FarmerHouse program. This can be achieved by transferring the remaining 28 tokens (equivalent to 28,000,000, considering the decimal configuration) from the PAW's Associated Token Account (ATA) to the ATA linked with the FarmsPDA. With this, the FarmsPDA is adequately financed to perform its duties within the FarmerHouse program.

### Minting and Configuring FieldNFT and TomatoNFT

Now, let's proceed to mint the FieldNFT and TomatoNFT. These two non-fungible tokens represent the fundamental elements of the FarmerHouse game - the farm field (FieldNFT) and the tomato plant (TomatoNFT).

During the minting process, ensure that the collection of TomatoNFT is set as TomatoSeedCollectionNFT, and for FieldNFT, its collection should be configured as FieldCollectionNFT. Feel free to mint these tokens directly to the FarmsPDA. 

> **Note:** Save the mintAddress, metadataAddress and masterEditionAddress of each.

Post minting, it's time to adjust the `updateAuthority` of the metadata for both these tokens. This adjustment involves setting the FarmsPDA as the new `updateAuthority`. This is a key step that empowers the FarmsPDA with the authority to update the metadata of these NFTs. 

This process is vital as it grants the FarmerHouse program the ability to regulate the growth stage of the TomatoNFT and make potential modifications to the FieldNFT. It's worth noting that these modifications can be made directly via the FarmerHouse program, eliminating the need for wallets to act as signers. The FarmsPDA, equipped with the signer seeds through the FarmerHouse program, can autonomously sign these updates.

Upon successful completion of these steps, you are all set to engage with the FarmerHouse program!


## Step 3: Creating the Escrow Constraint Model

The next step involves the creation and configuration of the Escrow Constraint Model account. This account will be storing the rules and constraints for our Trifle account, thus playing a pivotal role in the FarmerHouse program. In this step, we begin interacting with the Fusion program, so let's dive deeper into its mechanisms.

Within the FarmerHouse program, we've defined a method called `initializeConstraintModel`. This method is designed to create an Escrow Constraint Model account and append slots to this account, each with specific constraints. 

### Deriving the Escrow Constraint Model Address

The creation of the Escrow Constraint Model requires a new account address. This address is a Program Derived Address (PDA) generated through the Trifle smart contract, derived from the following seeds: 

- A byte-string of "escrow"
- The address of the creator of the Escrow Constraint Model, which in our case is PAW
- A byte-string of a name of our choice, for which we will use "Basic Farm"

Here's how you derive the address:

```javascript
const [escrowConstraintModelAddress] = PublicKey.findProgramAddressSync(
    [
        Buffer.from('escrow'),
        PAW.publicKey.toBuffer(),
        Buffer.from('Basic Farm'),
    ],
    new PublicKey(TRIFLE_PROGRAM_ADDRESS)
);
```

### Initializing the Constraint Model

Once the address is derived, you can initialize the Escrow Constraint Model using the `initializeConstraintModel` method. This method requires the following arguments:

- `name`: The same name we used to derive the Escrow Constraint Model address. In our case, it's "Basic Farm".
- `schema_uri`: A JSON schema for the Bundle (Trifle with main NFT plus slotted NFTs). More standardized and comprehensive information on using `schema_uri` will be updated in future versions of this guide.

And the following accounts:

- `escrowConstraintModel`: the Escrow Constraint Model address we just derived
- `farmsPda`: the FarmsPDA address
- `pdaAuthority` (Signer): the PAW address, which will fund the transaction
- `tomatoSeedCollectionMint`: the mint address for the TomatoSeedCollectionNFT
- `tomatoSeedCollectionMetadata`: the metadata address for the TomatoSeedCollectionNFT
- `trifleProgram`: the address for the Trifle Program from Metaplex Foundation
- `farmerHouseProgram`: the address for the FarmerHouse program
- `instructions`: The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor

This method will create the Escrow Constraint Model with PAW as its creator, enabling us to modify the model later by signing with PAW. Please note that, depending on the number of constraints (slots) you want to add to the model, this method may require a higher compute unit limit. For this example, we will use the maximum limit of 400,000 units.

### Adding Constraints to the Model

After creating the PDA, the method proceeds to add constraints to it. In the context of the Trifle program, a constraint is a set of rules for one slot in the Trifle structure. It outlines the name of the slot, the token limit amount for that slot, the type of constraint (either by collection, token, or none), and the transfer effect. If a constraint of type collection or token is created, only NFTs of that collection or token can be added to that slot.

In our scenario, we add six slots ("a1", "a2", "a3", "b1", "b2", "b3"), each with a collection type constraint, a token limit set to 1, and a transfer effect set to 1. Each of these slots represents a space in our farm field for a seed/plant to be planted in, providing a structural foundation for our game format.


## Step 4: Buying the Field and Seed NFTs

At this point in our NFT farming game, we progress to user interaction with the program, executing in-game actions. We start by acquiring the essential assets needed for the farming process, represented by FieldNFT and TomatoNFT. 

These assets are purchased from the FarmsPDA, the game shop, in exchange for CurrencyTokens. The process involves two key transactions:

1. **Buying a Field (FieldNFT)**
2. **Buying a Seed (TomatoNFT)**

### Buying the Field

The `buyField` method of the FarmerHouse program is invoked for this action. This method facilitates the purchase of the FieldNFT, transferring ownership from FarmsPDA to UserWallet. It also creates a TrifleAccount, which holds data about the NFT Bundle. This account is a Program Derived Address (PDA) owned by Trifle.

Several addresses must be derived for the execution of this method:

- `trifleAddress`: The Trifle PDA that stores information about the NFT Bundle. Derived from a byte-string of "escrow", the mint address of the parent NFT (fieldNft) in the bundle, and the TrifleAuthority (FarmsPDA in this case).
Here's how you derive the address:

```javascript
const [trifleAddress] = PublicKey.findProgramAddressSync(
    [
        Buffer.from('trifle'),
        fieldNft.mint.address.toBuffer(),
        farmsPda.toBuffer(),
    ],
    new PublicKey(TRIFLE_PROGRAM_ADDRESS)
);
```

- `escrowAccountAddress`: An account that holds NFT data without being the actual owner of the NFT. This allows an entity other than the NFT's owner to alter escrow data without modifying the NFT itself. Derived from a byte-string of "metadata", the TOKEN_METADATA_PROGRAM address, the mint address of the parent NFT in our bundle, a Uint8Array.from([1]), and the just derived TrifleAccount address.
Here's how you derive the address:

```javascript
const findEscrowPda = (mint: PublicKey, authority: number, creator?: PublicKey) => {
    let seeds = [
        Buffer.from('metadata'),
        new PublicKey(TM_PROGRAM_ADDRESS).toBuffer(),
        mint.toBuffer(),
        Uint8Array.from([authority]),
    ];

    if (authority === 1) {
        if (creator) {
            seeds.push(creator.toBuffer());
        } else {
            throw new Error("Creator is required");
        }
    }

    seeds.push(Buffer.from('escrow'));
    return PublicKey.findProgramAddressSync(
        seeds,
        new PublicKey(TM_PROGRAM_ADDRESS)
    );
}
const [escrowAccountAddress] = findEscrowPda(fieldNft.mint.address, 1, trifleAddress);
```

- `splAtaSource`: The Associated Token Account (ATA) of the CurrencyToken linked to UserWallet. This acts as the source of currency to purchase the FieldNFT.

- `splAtaDestination`: The ATA of the CurrencyToken linked to FarmsPDA. This is the destination for the paid currency in exchange for FieldNFT.

- `fieldAtaSource`: The ATA of the FieldNFT linked to FarmsPDA. This serves as the source of the FieldNFT being purchased.

- `fieldAtaDestination`: The ATA of the FieldNFT linked to UserWallet. This will be the destination of the purchased FieldNFT.

With these derived addresses, the `buyField` method can be invoked, passing the following accounts:

- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this is the signer and payer for the transaction.
- `splAtaSource`: The address where the payment for the NFT will come from.
- `splAtaDestination`: The address where the payment for the NFT will go.
- `splMint`: The mint address of the CurrencyToken.
- `fieldAtaSource`: The source address for the FieldNFT being purchased.
- `fieldAtaDestination`: The destination address for the FieldNFT being purchased.
- `fieldMint`: The mint address of the FieldNFT.
- `fieldMetadata`: The metadata address of the FieldNFT.
- `fieldMasterEdition`: The Master Edition address of the FieldNFT.
- `farmerHouseProgram`: The address of the FarmerHouse program.
- `escrowAccount`: The address of the escrow account.
- `escrowConstraintModel`: The address of the Escrow Constraint Model created in the previous steps.
- `trifleAccount`: The address for the PDA owned by Trifle that holds the NFT Bundle information.
- `tokenMetadataProgram`: The address of the Token Metadata Program from Metaplex Foundation.
- `trifleProgram`: The address of the Trifle Program from Metaplex Foundation.
- `instructions`: The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor.

This method call accomplishes the purchase of the FieldNFT by transferring 70 (or 70,000,000) CurrencyTokens from UserWallet to FarmsPDA. It also transfers the FieldNFT from FarmsPDA to UserWallet and creates the TriflePDA account via the Trifle program.

### Buying the Seed

The next phase is to buy the seed of the plant we aim to farm. The `buySeed` method is invoked for this purpose. This method handles the purchase of the TomatoNFT (in its seed stage) from FarmsPDA to UserWallet, using CurrencyTokens as payment.

A couple of addresses associated with TomatoNFT ATAs need to be derived for this method:

- `seedAtaSource`: The ATA of the TomatoNFT linked to FarmsPDA. This acts as the source for the TomatoNFT being purchased.

- `seedAtaDestination`: The ATA of the TomatoNFT linked to UserWallet. This will be the destination of the purchased TomatoNFT.

The `buySeed` method requires the following accounts:

- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this acts as the signer and payer for the transaction.
- `splAtaSource`: The address where the payment for the NFT will come from.
- `splAtaDestination`: The address where the payment for the NFT will go.
- `splMint`: The mint address of the CurrencyToken.
- `seedAtaSource`: The source address for the TomatoNFT being purchased.
- `seedAtaDestination`: The destination address for the TomatoNFT being purchased.
- `seedMint`: The mint address of the TomatoNFT.
- `seedMetadata`: The metadata address of the TomatoNFT.
- `farmerHouseProgram`: The address of the FarmerHouse program.

On execution of `buySeed`, we effect a transfer of 2 (or 2,000,000) CurrencyTokens from UserWallet to FarmsPDA, as well as the TomatoNFT from FarmsPDA to UserWallet.


## Step 5: Planting the Seed in the Farm Field

Now that we have both the farm field and the seed, our next task is to plant the seed. In our NFT farming game, this is achieved by invoking the "transfer_in" instruction from the Trifle Program. Our FarmerHouse interface provides the method `plantSeed` for this action.

This operation requires the derivation of one new address:

- `attributeDstTokenAccount`: The Associated Token Account (ATA) corresponding to the TomatoNFT mint address and the EscrowAccount. This account will be the destination for the TomatoNFT being transferred.

This method requires the following argument:
- `crop_slot`: Represents the slot on the farm field where we aim to plant the TomatoNFT in its seed stage. We will use the string "a1" for this purpose.

And the following accounts:
- `trifleAccount`: The address for the PDA owned by Trifle that holds the NFT Bundle information.
- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this is the signer and payer for the transaction.
- `escrowConstraintModel`: Address of the Escrow Constraint Model that sets the rules for the TrifleAccount.
- `escrowAccount`: The address of the escrow account.
- `fieldMint`: The mint address of the FieldNFT.
- `fieldTokenAccount`: The ATA of FieldNFT with the UserWallet.
- `fieldMasterEdition`: The master edition address of the FieldNFT.
- `attributeMint`: The mint address of the TomatoNFT.
- `attributeSrcTokenAccount`: The ATA of TomatoNFT with the UserWallet.
- `attributeDstTokenAccount`: The ATA of TomatoNFT with the EscrowAccount.
- `attributeMetadata`: The metadata address of the TomatoNFT.
- `tokenProgram`: Address for the Solana's SPL Token Program.
- `associatedTokenProgram`: Address for the Solana's Associated Token Program.
- `tokenMetadataProgram`: Address for the Metaplex Foundation's Token Metadata Program.
- `farmerHouseProgram`: Address for the FarmerHouse program.
- `trifleProgram`: Address for the Metaplex Foundation's Trifle Program.

This `plantSeed` method transfers the TomatoNFT (which must be part of the `tomatoSeedCollection` due to the constraints set by our `escrowConstraintModel`) to the `attributeDstTokenAccount` that we derived from the EscrowAccount, that in turn is derived from the TrifleAccount.

As a result, our TrifleAccount (representing the farm field parent NFT) now "holds" the TomatoNFT (representing the seed child NFT), creating the NFT bundle. Therefore, our parent NFT effectively has a planted seed, symbolizing the beginning of the farming process in the game.


## Step 6: Watering the Crop

As our virtual farming game advances, we need to nurture our planted seed. This is done by watering the crop, which in our case involves updating the collection in the metadata of the TomatoNFT to represent a higher growth stage. 

The "water" method in our FarmerHouse interface facilitates this. It changes the `tomatoSeedCollection` to `tomatoSapplingCollection` and subsequently to `ripeTomatoCollection`. This transition is carried out automatically each time we call this method.

This method takes the following argument:
- `crop_slot` (Argument): The slot in which the TomatoNFT in its current growth stage is located. As we're continuing from the previous step, we use "a1" for this purpose.

And the following accounts:
- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this is the signer and payer for the transaction.
- `fieldMint`: The mint address of the FieldNFT.
- `fieldAssociatedToken`: The Associated Token Account (ATA) of FieldNFT linked to UserWallet.
- `cropMetadata`: The metadata address of the TomatoNFT.
- `trifleAccount`: The Program Derived Account (PDA) owned by Trifle that holds the NFT Bundle information.
- `tokenMetadataProgram`: The address for the Metaplex Foundation's Token Metadata Program.
- `farmerHouseProgram`: The address for the FarmerHouse program.

With each invocation of the `water` method, the collection of the TomatoNFT advances to a higher growth stage. Since FarmsPDA is the update authority for the TomatoNFT, we can update the metadata from within our program successfully.

This watering process can be repeated until the collection is set to `ripeTomatoCollection`. Once this stage is reached, the TomatoNFT cannot be watered further, and it should be harvested, marking the completion of the growth cycle.


## Step 7: Harvesting the Crop

The final phase of our virtual farming process is harvesting the crop, i.e., the TomatoNFT. We accomplish this through the `harvestCrop` method in our FarmerHouse program. This method invokes the `transfer_out` instruction of the Trifle Program, removing the TomatoNFT from the escrowAccount (linked to the TrifleAccount) and transferring it to the UserWallet.

This method takes the following argument:
- `crop_slot` (Argument): The slot in which the TomatoNFT in its current growth stage is located. As we're continuing from the previous step, we use "a1" for this purpose.

And the following accounts:
- `trifleAccount`: The address for the PDA owned by Trifle that holds the NFT Bundle information.
- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this is the signer and payer for the transaction.
- `escrowConstraintModel`: The address of the Escrow Constraint Model created in the previous steps.
- `escrowAccount`: The address of the escrow account.
- `fieldMint`: The mint address of the FieldNFT.
- `fieldTokenAccount`: The Associated Token Account (ATA) of FieldNFT linked to UserWallet.
- `fieldEdition`: The Master Edition address of the FieldNFT.
- `fieldMetadata`: The metadata address of the FieldNFT.
- `attributeMint`: The mint address of the TomatoNFT.
- `attributeSrcTokenAccount`: The ATA of TomatoNFT with the escrowAccount.
- `attributeDstTokenAccount`: The ATA of TomatoNFT with the UserWallet.
- `attributeMetadata`: The metadata address of the TomatoNFT.
- `tokenProgram`: The address for Solana's SPL Token Program.
- `associatedTokenProgram`: The address for Solana's Associated Token Program.
- `tokenMetadataProgram`: The address for the Metaplex Foundation's Token Metadata Program.
- `farmerHouseProgram`: The address for the FarmerHouse program.
- `trifleProgram`: The address for the Metaplex Foundation's Trifle Program.
- `instructions`: The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor.

Note the switch between `attributeSrcTokenAccount` and `attributeDstTokenAccount` in comparison to the previous stages. This reflects the transfer of the TomatoNFT from the EscrowAccount to the UserWallet.

With the execution of this function, we successfully complete the virtual farming process for the TomatoNFT. The seed has grown to a ripe tomato, now owned by the UserWallet, ready to be sold back to the shop (represented by the FarmsPDA).


## Step 8: Selling the Crop

The final step in our farming process involves selling the fully grown TomatoNFT back to the shop. Given that the TomatoNFT has grown, it can now be sold at a higher price than the initial purchase, thus yielding a profit for the UserWallet. This process is executed through the `sellCrop` method in our FarmerHouse interface.

The `sellCrop` method checks the growth stage of the crop by comparing its current collection to the `tomatoCollections` stored in the `farmsPda` during initialization. The selling price varies according to the growth stage:

- Seed stage: The selling price is the same as the buying price (2 CurrencyTokens or 2,000,000 subunits).
- Sapling stage: The selling price is 3.5 CurrencyTokens (or 3,500,000 subunits).
- Ripe stage: The selling price is 7 CurrencyTokens (or 7,000,000 subunits).

This method requires the following accounts:
- `farmsPda`: The address of the FarmsPDA.
- `userAccount`: The address of the UserWallet; this is the signer and payer for the transaction.
- `splAtaSource`: The address from which the payment for the NFT will be withdrawn.
- `splAtaDestination`: The address to which the payment for the NFT will be sent.
- `splMint`: The mint address of the CurrencyToken.
- `cropAtaSource`: The source address for the TomatoNFT being sold.
- `cropAtaDestination`: The destination address for the TomatoNFT being sold.
- `cropMetadata`: The metadata address of the TomatoNFT.
- `farmerHouseProgram`: The address of the FarmerHouse program.

> **Note:** Take note that the transactions now involve sending CurrencyTokens from the `farmsPda` to the `UserWallet` and transferring the TomatoNFT from the `UserWallet` to the `farmsPda`. Therefore, all Associated Token Accounts (ATAs) need to be properly set up. Remember that these accounts' configuration is reversed from when we bought the TomatoNFT, so you must ensure you're providing the correct ATAs.

With this step, we have successfully completed the entire farming process and made a profit in our UserWallet. However, keep in mind that the Fusion Program's usage and implementation can vary based on specific requirements and intentions. In the following sections, we will discuss potential modifications, as well as safety checks and concerns you should consider when using the program in this manner.


## Safety Checks and Measures


When developing blockchain applications, it's essential to keep security and safety measures at the forefront of your process. Here are some important points to keep in mind:

1. **Consistent Program Usage**
 
One of the most critical checks involves ensuring the consistency of the programs used in the transaction. As a developer, you need to validate the program ID against the intended one. This validation prevents malicious actors from injecting their programs and creating unforeseen vulnerabilities.

2. **Account and Data Verification**

If your methods involve changing metadata or program-derived addresses (PDA) data, it's vital to verify all the accounts and data involved. This task might sound daunting, but fear not: using frameworks like Anchor for Solana makes this task more manageable. Anchor provides macros that ensure the PDAs are valid, taking into account their specific seeds and bump.

3. **Transaction Atomicity and Error Handling**

In Solana, a transaction is atomic, meaning it will either completely succeed or fail; there's no in-between state. This atomicity can be a safety net, but developers need to be aware of its implications.

For instance, if a method involves multiple cross-program invocations, the atomicity of the transaction implies that if any of these invocations fail, the whole transaction will fail. In such cases, any state changes or fund transfers made in previous successful invocations will be reverted.

This means that if a user was sending funds in the first invocation and was supposed to receive an item in the next invocation, a failure in the second invocation would result in the user losing their funds.

It's crucial to handle errors properly to prevent such situations. Error handling should include validating all account and data elements involved in later transactions, ensuring their accuracy before the first transaction is even attempted.

   
4. **Redundancy**

In some scenarios, redundancy can play a key role in maintaining system integrity. For example, although some accounts and data are already verified by third-party programs, a developer can add another layer of validation.

This redundancy can be particularly beneficial in methods involving several cross-program invocations. The aim is to prevent a situation where the first invocation transfers assets from the user, but a failure in a subsequent invocation leaves the user at a loss.

In conclusion, ensuring the safety of your blockchain programs requires careful design, robust validation mechanisms, and precise error handling. The effort is worthwhile because it helps to maintain the trust of users, which is fundamental to the success of any blockchain project.


## Potential Modifications


After getting comfortable with the farming program, you might start thinking about what more you can do with it. Fusion, the NFT protocol we're utilizing here, offers a flexible structure to build upon. Let's explore some possible alterations and additions:

1. **Diverse NFT Relationships**

Our current setup features a 1-parent to many-children NFT relationship. This parent NFT ('Field') can hold various children NFTs ('TomatoNft'). However, Fusion provides opportunities for diverse relationships:

- Slot-Filling Structure: Instead of just adding child NFTs to a parent, consider using a slot-filling structure. This could involve specific slots in the parent NFT being occupied by Semi-Fungible Tokens (SFTs). As slots are filled, the parent NFT's state, including its image and name, could evolve.

- Bundle Assets: Bundle several NFTs to be traded or used as a single unit. In the current setup, we treat NFTs independently. However, Fusion (Trifle) verifies the owner of the child NFT only during "transfer in" and "transfer out" operations. Given that our program's PDA holds the authority for the trifle, if we sell the parent NFT to a different user, the child NFTs within the Trifle still belong to it. Therefore, the whole Trifle is effectively "transferred" to the new user, which makes it simple to trade bundled assets.

2. **Time-Based Actions**

In our example, changes in the crop's growth stages are instantaneous, with each watering session immediately advancing the stage. For more realism, you can integrate Solana's on-chain clock to implement time-based growth stages. Crops could then require certain time periods to grow, making the farming process more engaging and realistic.

3. **Random Events**

Introducing randomness to your program can make it more engaging and fun. Random events such as weather fluctuations, pest infestations, or market price changes can significantly influence farming outcomes. You can generate randomness on-chain using Verifiable Random Function (VRF) or devise the randomness logic off-chain for simplicity. Each approach has its pros and cons that should be considered.

Remember that these modifications will require additional development and testing. As you build upon the base program, always ensure your changes are secure and improve the user experience. The possibilities with Fusion are extensive, and with some creativity, you can make a unique and engaging project. Good luck and happy farming!


## Accounts

Understanding the structure of key accounts in the FarmerHouse program is crucial for interacting with it effectively. Here, we'll delve into the details of the `FarmsPda` PDA.


### FarmsPda
The `FarmsPda` PDA holds the critical information for the FarmerHouse program. This includes addresses of the collections, the CurrencyToken mint address, and the PDA Authority address. It also acts as a "shop" for farm fields and seeds/plants.

| Field                                 | Offset    | Size      | Description
| ------------------------------------- | ------    | ----      | --
| &mdash;                               | 0         | 8         | Anchor account discriminator.
| `bump`                                | 8         | 1         | The bump of PDA stored as `u8`.
| `authority_address`                   | 9         | 32        | `Pubkey` of the PDA Authority (PAW).
| `spl_mint_address`                    | 41        | 32        | `Pubkey` of the CurrencyToken Mint.
| `tomato_seed_collection_address`      | 73        | 32        | `Pubkey` of the TomatoSeedCollection Mint.
| `tomato_sappling_collection_address`  | 105       | 32        | `Pubkey` of the TomatoSaplingCollection Mint.
| `ripe_tomato_collection_address`      | 137       | 32        | `Pubkey` of the RipeTomatoCollection Mint.
| `field_collection_address`            | 169       | 32        | `Pubkey` of the FieldCollection Mint.

In the next section, we will discuss the instructions of the FarmerHouse program in more detail.


## Instructions

The FarmerHouse program consists of a sequence of specific instructions. Understanding these instructions and how they function will provide you with a clear grasp of how FarmerHouse operates. In this section, we'll explore the key instructions, beginning with `initializeFarmsPda()`.


### initializeFarmsPda()

This instruction is instrumental in initializing the `farmsPda` PDA, a central component in the FarmerHouse program.

<details>
  <summary>Accounts</summary>

| Name                          | Writable | Signer | Description |
| ----------------------------- | :------: | :----: | -- |
| `pda_authority`               |    ✅    |   ✅   | The PAW, registered as the authority for the `farmsPda`. |
| `farms_pda`                   |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `pda_associated_token_account`|    ✅    |        | The ATA for the `farmsPda` and `splMint`. |
| `spl_mint`                    |          |        | The mint address of the `CurrencyToken`. |
| `farmer_house_program`        |          |        | The programId of FarmerHouse. |

</details>

<details>
  <summary>Arguments</summary>

| Argument                              | Type    | Description |
| ------------------------------------- | ------  | -- |
| `tomato_seed_collection_address`      | `Pubkey`| The public key for the TomatoSeedCollection Mint. |
| `tomato_sapling_collection_address`   | `Pubkey`| The public key for the TomatoSaplingCollection Mint. |
| `ripe_tomato_collection_address`      | `Pubkey`| The public key for the RipeTomatoCollection Mint. |
| `field_collection_address`            | `Pubkey`| The public key for the FieldCollection Mint. |

</details>


### initConstraintModel()

This instruction initializes the constraint model in Trifle with 6 `collection_constraints`, essentially creating 6 slots with a collection constraint. This implies that the collection for the NFTs placed into these slots must match the one set during this initialization step, which is the `tomatoSeedCollection`. Furthermore, each of these slots is designed to hold only a single token.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `escrow_constraint_model`             |    ✅    |        | Derived from the PAW address and the string "Basic Farm". This model will guide the constraints for the slots. |
| `farms_pda`                           |          |        | The `farmsPda`, storing all the essential information for the program. |
| `pda_authority`                       |    ✅    |   ✅   | The PAW, which is registered as the authority for the farmsPda. |
| `tomato_seed_collection_metadata`     |          |        | The metadata for the TomatoSeedCollection NFT. |
| `tomato_seed_collection_mint`         |          |        | The mint for the TomatoSeedCollection NFT. |
| `trifle_program`                      |          |        | The address of mpl-trifle from metaplex-foundation. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |
| `instructions`                        |          |        | The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor. |

</details>

<details>
  <summary>Arguments</summary>

| Argument                              | Type    | Description |
| ------------------------------------- | ------  | -- |
| `name`         | `String`| A "Basic Farm" string, or any other string consistent with the string the escrow_constraint_model address was derived from. |
| `schema_uri`   | `String`| The schema uri. |

</details>


### buyField()

This instruction executes a series of operations in a specific order:

1. If necessary, creates the associated token account (ATA) for the user and `fieldNft`.
2. Transfers 70 (or 70,000,000 due to decimals) currencyTokens from `user_account` to `farms_pda`.
3. Transfers the `fieldNft` from `farms_pda` to `user_account`.
4. Creates the Trifle account using the `fieldNft` and `farms_pda` as authority.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `farms_pda`                           |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `user_account`                        |    ✅    |   ✅   | The UserWallet initiating and paying the transaction. |
| `spl_ata_source`                      |    ✅    |        | The ATA of the currencyToken with UserWallet. |
| `spl_ata_destination`                 |    ✅    |        | The ATA of the currencyToken with FarmsPda. |
| `spl_mint`                            |          |        | The mint address of the currencyToken. |
| `field_ata_source`                    |    ✅    |        | The ATA of the FieldNft with FarmsPda. |
| `field_ata_destination`               |    ✅    |        | The ATA of the FieldNft with UserWallet. |
| `field_mint`                          |          |        | The mint address of FieldNft. |
| `field_metadata`                      |    ✅    |        | The metadata address of FieldNft. |
| `field_master_edition`                |          |        | The master_edition of FieldNft. |
| `escrow_account`                      |    ✅    |        | The escrow address for FieldNft and trifleAddress. |
| `escrow_constraint_model`             |    ✅    |        | The escrow constraint model derived from the PAW address and the string "Basic Farm" and containing the rules for the trifle PDA. |
| `trifle_account`                      |    ✅    |        | The Trifle account, which maps NFTs to their slots based on the Constraint Model and manages their states. Derived from FieldNft and FarmsPda. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |
| `trifle_program`                      |          |        | The address of mpl-trifle from metaplex-foundation. |
| `token_metadata_program`              |          |        | The address of mpl-token-metadata from metaplex-foundation. |
| `instructions`                        |          |        | The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor. |

</details>

Note: This instruction doesn't require any arguments.

### buySeed()

This instruction executes a series of operations in a specific order:

1. If necessary, creates the associated token account (ATA) for the user and `TomatoNft`.
2. Transfers 2 (or 2,000,000 due to decimals) currencyTokens from `user_account` to `farms_pda`.
3. Transfers the `TomatoNft` from `farms_pda` to `user_account`.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `farms_pda`                           |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `user_account`                        |    ✅    |   ✅   | The UserWallet initiating and paying the transaction. |
| `spl_ata_source`                      |    ✅    |        | The ATA of the currencyToken with UserWallet. |
| `spl_ata_destination`                 |    ✅    |        | The ATA of the currencyToken with FarmsPda. |
| `spl_mint`                            |          |        | The mint address of the currencyToken. |
| `seed_ata_source`                     |    ✅    |        | The ATA of the TomatoNft with FarmsPda. |
| `seed_ata_destination`                |    ✅    |        | The ATA of the TomatoNft with UserWallet. This is created if it doesn't already exist. |
| `seed_mint`                           |          |        | The mint address of TomatoNft. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |

</details>

Note: This instruction doesn't require any arguments.


### sellCrop()

This instruction performs a series of operations in a specific order:

1. Transfers a certain amount of currencyTokens from `farms_pda` to `user_account`. The transferred amount depends on the growth stage of the TomatoNft: 3.5 currencyTokens (or 3,500,000 due to decimals) if the TomatoNft has grown to sapling, and 7 currencyTokens (or 7,000,000 due to decimals) if the TomatoNft has grown to ripe.
2. Transfers the `TomatoNft` from `user_account` to `farms_pda`.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `farms_pda`                           |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `user_account`                        |    ✅    |   ✅   | The UserWallet initiating and paying the transaction. |
| `spl_ata_source`                      |    ✅    |        | The ATA of the currencyToken with FarmsPda. |
| `spl_ata_destination`                 |    ✅    |        | The ATA of the currencyToken with UserWallet. |
| `spl_mint`                            |          |        | The mint address of the currencyToken. |
| `crop_ata_source`                     |    ✅    |        | The ATA of the TomatoNft with UserWallet. |
| `crop_ata_destination`                |    ✅    |        | The ATA of the TomatoNft with FarmsPda. |
| `crop_metadata`                       |          |        | The metadata address of TomatoNft. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |

</details>

Note: This instruction doesn't require any arguments.


### plantSeed()

This instruction transfers the `tomatoSeed` NFT into the Trifle in the slot specified in the `crop_slot` argument. 

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `trifle_account`                      |    ✅    |        | The Trifle account, which maps NFTs to their slots based on the Constraint Model and manages their states. Derived from FieldNft and FarmsPda. |
| `farms_pda`                           |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `user_account`                        |    ✅    |   ✅   | The UserWallet initiating and paying the transaction. |
| `escrow_constraint_model`             |    ✅    |        | The escrow constraint model derived from the PAW address and the string "Basic Farm" and containing the rules for the trifle PDA. |
| `escrow_account`                      |          |        | The escrow address for FieldNft and trifleAddress. |
| `field_mint`                          |    ✅    |        | The mint address of the fieldNft. |
| `field_token_account`                 |    ✅    |        | The ATA of the FieldNft with UserWallet. |
| `field_edition`                       |    ✅    |        | The master edition of the fieldNft. |
| `attribute_mint`                      |    ✅    |        | The mint address of the tomatoNft. |
| `attribute_src_token_account`         |    ✅    |        | The ATA of the TomatoNft with UserWallet. |
| `attribute_dst_token_account`         |    ✅    |        | The ATA of the TomatoNft with EscrowAccount. |
| `attribute_metadata`                  |    ✅    |        | The metadata of the tomatoNft. |
| `token_metadata_program`              |          |        | The address of mpl-token-metadata from metaplex-foundation. |
| `associated_token_program`            |          |        | The address of the Associated Token program. |
| `token_program`                       |          |        | The address of the SPL Token program. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |
| `trifle_program`                      |          |        | The address of mpl-trifle from metaplex-foundation. |

</details>

<details>
  <summary>Arguments</summary>
  
| Name        | Type   | Description
| ----------- | ------ | --
| `crop_slot` | String | The slot to which we want to transfer the Nft. It must be one of the following: "a1", "a2", "a3", "b1", "b2", "b3".

</details>


### Water()

This instruction matures the `tomatoNft` by one growth tier in the Trifle. For example, from seed to sapling and from sapling to ripe.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description |
| ------------------------------------- | :------: | :----: | -- |
| `farms_pda`                           |    ✅    |        | The `farmsPda`, storing all the essential information for the program. |
| `user_account`                        |    ✅    |   ✅   | The UserWallet initiating and paying the transaction. |
| `field_mint`                          |    ✅    |        | The mint address of the FieldNft. |
| `field_associated_token`              |    ✅    |        | The ATA of the FieldNft and the UserWallet. |
| `crop_metadata`                       |    ✅    |        | The metadata account of the `tomatoNft`. |
| `trifle_account`                      |          |        | The Trifle account, which maps NFTs to their slots based on the Constraint Model and manages their states. Derived from FieldNft and FarmsPda. |
| `token_metadata_program`              |          |        | The address of mpl-token-metadata from metaplex-foundation. |
| `farmer_house_program`                |          |        | The address of the FarmerHouse program. |

</details>

<details>
  <summary>Arguments</summary>
  
| Name        | Type   | Description
| ----------- | ------ | --
| `crop_slot` | String | The slot in which the Nft that needs to be watered is in the trifle PDA. It must be one of the following: "a1", "a2", "a3", "b1", "b2", "b3".

</details>


### HarvestCrop()

This instruction transfers the `tomatoNft` NFT out of the Trifle from the slot specified in the `crop_slot` argument, back into the User Wallet.

<details>
  <summary>Accounts</summary>
  
| Name                                  | Writable | Signer | Description
| ------------------------------------- | :------: | :----: | --
| `trifle_account`                      |    ✅    |        | The Trifle account, which maps NFTs to their slots based on the Constraint Model and manages their states.
| `farms_pda`                           |    ✅    |        | The PDA for the farms, created using the Farmer House program seed and bump.
| `user_account`                        |    ✅    |   ✅   | The user's wallet account, which signed the transaction.
| `escrow_constraint_model`             |    ✅    |        | The Constraint Model account, which provides the rules for mapping slots to NFTs in the Trifle account.
| `escrow_account`                      |          |        | The escrow account for the operation.
| `field_mint`                          |    ✅    |        | The mint address of the FieldNft.
| `field_token_account`                 |    ✅    |        | The ATA of the FieldNft and the user's wallet.
| `field_metadata`                      |    ✅    |        | The metadata account of the FieldNft.
| `field_edition`                       |    ✅    |        | The master edition address of FieldNft.
| `attribute_mint`                      |          |        | The mint address of the TomatoNft.
| `attribute_src_token_account`         |    ✅    |        | The ATA of the TomatoNft and the Trifle account.
| `attribute_dst_token_account`         |    ✅    |        | The ATA of the TomatoNft and the user's wallet.
| `attribute_metadata`                  |          |        | The metadata address of TomatoNft.
| `token_metadata_program`              |          |        | The program ID of the Token Metadata program (Metaplex).
| `associated_token_program`            |          |        | The program ID of the Associated Token program.
| `token_program`                       |          |        | The program ID of the SPL Token program.
| `farmer_house_program`                |          |        | The program ID of the Farmer House program.
| `trifle_program`                      |          |        | The program ID of the Trifle program.
| `instructions`                        |          |        | The `SYSVAR_INSTRUCTIONS_PUBKEY` from Solana/Anchor.

</details>

<details>
  <summary>Arguments</summary>
  
| Name        | Type   | Description
| ----------- | ------ | --
| `crop_slot` | String | The slot in the Trifle PDA where the Nft to be harvested is located. It must be one of the following: "a1", "a2", "a3", "b1", "b2", "b3".

</details>

