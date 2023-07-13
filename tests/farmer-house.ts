import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FarmerHouse } from "../target/types/farmer_house";
import * as metadata from "@metaplex-foundation/mpl-token-metadata";
import * as trifle from "@metaplex-foundation/mpl-trifle";
import { createMint, getAccount, getAssociatedTokenAddress, mintTo, transfer, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { Metaplex, keypairIdentity, Sft, toBigNumber, SplTokenAmount, SftWithToken, NftWithToken, Nft, token } from "@metaplex-foundation/js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import * as assert from "assert";
import { findEscrowConstraintModelPda, findTriflePda, findEscrowPda } from "../helpers/pdas";
import lumina from '@lumina-dev/test';

lumina();

function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

describe("farmer-house", () => {
  let currencyMint: anchor.web3.PublicKey;
  let fieldCollection: NftWithToken;
  let tomatoSeedCollection: NftWithToken;
  let tomatoSapplingCollection: NftWithToken;
  let ripeTomatoCollection: NftWithToken;
  let fieldNft: Nft;
  let tomatoNft: Nft;

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FarmerHouse as Program<FarmerHouse>;
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  // Create ProgramAdminWallet (PAW)
  const PAW = provider.wallet as anchor.Wallet;

  const userWallet = anchor.web3.Keypair.generate();

  // Configure Metaplex with PAW in order to mint all farm collections and nfts
  const metaplex = Metaplex.make(provider.connection)
    .use(keypairIdentity(PAW.payer));

  const [farmsPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('farmer-house-farms'),
      program.programId.toBuffer(),
    ],
    program.programId
  );

  console.log("\n")
  console.log(`PAW address: ${PAW.publicKey.toBase58()}`)
  console.log(`User address: ${userWallet.publicKey.toBase58()}`)
  console.log(`FarmsPda address: ${farmsPda.toBase58()}`)

  const escrowModelName = "Basic Farm"
  const escrowModelSchemaUri = "https://fossil-test.fra1.digitaloceanspaces.com/the_search_schema.json"
  const [escrowConstraintModelAddress] = findEscrowConstraintModelPda(PAW.publicKey, escrowModelName);

  before("Airdrop", async () => {

    const airdropSignature = await provider.connection.requestAirdrop(userWallet.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });

    // Checks balance of newly generated user wallet after airdrop
    const balance = await provider.connection.getBalance(userWallet.publicKey);

    assert.equal(1000000000, balance);
  });

  it("Mints Token and transfers to user", async () => {

    const tokenMint = await createMint(
      provider.connection,
      PAW.payer,
      PAW.publicKey,
      PAW.publicKey,
      6
    );

    console.log("Mint Address:", tokenMint.toBase58());

    const creatorTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      PAW.payer,
      tokenMint,
      PAW.publicKey,
    );

    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      userWallet,
      tokenMint,
      userWallet.publicKey
    );

    await mintTo(
      provider.connection,
      PAW.payer,
      tokenMint,
      creatorTokenAccount.address,
      PAW.publicKey,
      1000000000
    );

    console.log("Tokens minted to PAW's token account");

    await transfer(
      provider.connection,
      PAW.payer,
      creatorTokenAccount.address,
      userTokenAccount.address,
      PAW.publicKey,
      300000000
    );

    const userTokenAccountInfo = await getAccount(provider.connection, userTokenAccount.address);

    assert.equal(300000000, userTokenAccountInfo.amount);
    console.log("300 Tokens transferred to users's token account");

    currencyMint = tokenMint;

  })

  it("Mints Collections", async () => {

    console.log("Minting field collection")
    const { nft: fieldCollectionNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Field Collection",
      symbol: "FIELDCO",
      sellerFeeBasisPoints: 0,
      isCollection: true,
    });

    console.log("Minting tomato seed collection")
    const { nft: tomatoSeedCollectionNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Tomato Seed Collection",
      symbol: "TOSECO",
      sellerFeeBasisPoints: 0,
      isCollection: true
    });

    console.log("Minting young tomato plant collection")
    const { nft: tomatoSapplingCollectionNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Young Tomato Plant Collection",
      symbol: "TOPLCO",
      sellerFeeBasisPoints: 0,
      isCollection: true
    });

    console.log("Minting ripe tomato collection")
    const { nft: ripeTomatoCollectionNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Ripe Tomato Collection",
      symbol: "RITOCO",
      sellerFeeBasisPoints: 0,
      isCollection: true
    });

    fieldCollection = fieldCollectionNft;
    tomatoSeedCollection = tomatoSeedCollectionNft;
    tomatoSapplingCollection = tomatoSapplingCollectionNft;
    ripeTomatoCollection = ripeTomatoCollectionNft;

    console.log("All NFT collecitons minted")
  })

  it("Initializes Farms Pda", async () => {

    console.log("Initializing Farms PDA");

    const farmsPdaTokenAccount: anchor.web3.PublicKey = await getAssociatedTokenAddress(currencyMint, farmsPda, true);

    const initializeSignature = await program.methods.initializeFarmsPda(
      tomatoSeedCollection.mint.address, tomatoSapplingCollection.mint.address, ripeTomatoCollection.mint.address, fieldCollection.mint.address
    )
      .accounts({
        pdaAuthority: PAW.publicKey,
        farmsPda,
        pdaAssociatedTokenAccount: farmsPdaTokenAccount,
        splMint: currencyMint,
        farmerHouseProgram: program.programId,
      })
      .signers([PAW.payer])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: initializeSignature,
    });

    console.log("Farms PDA Initialized");

    const creatorTokenAccountAddress = await getAssociatedTokenAddress(currencyMint, PAW.publicKey)

    await transfer(
      provider.connection,
      PAW.payer,
      creatorTokenAccountAddress,
      farmsPdaTokenAccount,
      PAW.publicKey,
      300000000
    );

    const farmsPdaTokenAccountInfo = await getAccount(provider.connection, farmsPdaTokenAccount);
    assert.equal(300000000, farmsPdaTokenAccountInfo.amount);
    console.log("300 Tokens transferred to shop's token account");
  })

  it("Mints necessary NFTs and update authority for collections metadata", async () => {

    console.log("Minting field")
    const { nft: newFieldNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Basic Field",
      symbol: "FIELD",
      sellerFeeBasisPoints: 0,
      tokenOwner: farmsPda,
      isMutable: true,
      collection: fieldCollection.address,
      collectionAuthority: PAW.payer
    })

    metaplex.nfts().update({
      nftOrSft: newFieldNft,
      newUpdateAuthority: farmsPda,
    })

    console.log("Minting tomato seed")
    const { nft: newTomatoNft } = await metaplex.nfts().create({
      uri: "https://fossil-test.fra1.digitaloceanspaces.com/caveman-test-metadata.json",
      name: "Tomato Seed",
      symbol: "TOMATO",
      sellerFeeBasisPoints: 0,
      tokenOwner: farmsPda,
      isMutable: true,
      collection: tomatoSeedCollection.address,
      collectionAuthority: PAW.payer
    })

    metaplex.nfts().update({
      nftOrSft: newTomatoNft,
      newUpdateAuthority: farmsPda,
    })

    fieldNft = newFieldNft;
    tomatoNft = newTomatoNft;

    await metaplex.nfts().findAllByOwner({ owner: farmsPda }).then(data => data.length == 2 && console.log("All necessary NFTs successfully minted to farmShopPda"));
  })

  it("Create escrow constraint model", async () => {

    console.log("Creating constraint model");

    const createEscrowSignature = await program.methods.initializeConstraintModel(escrowModelName, escrowModelSchemaUri)
      .accounts({
        escrowConstraintModel: escrowConstraintModelAddress,
        farmsPda,
        pdaAuthority: PAW.publicKey,
        tomatoSeedCollectionMint: tomatoSeedCollection.mint.address,
        tomatoSeedCollectionMetadata: tomatoSeedCollection.metadataAddress,
        trifleProgram: trifle.PROGRAM_ID,
        farmerHouseProgram: program.programId,
        instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .signers([PAW.payer])
      .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 }), anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 0 })])
      .rpc();

    const createEscrowBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: createEscrowBlockHash.blockhash,
      lastValidBlockHeight: createEscrowBlockHash.lastValidBlockHeight,
      signature: createEscrowSignature,
    });

  })

  it("Buys Field and creates trifle account with field", async () => {
    // Get ATA address for currencyMint + userWallet (initialized)
    const splAtaSource = await getAssociatedTokenAddress(currencyMint, userWallet.publicKey);
    // Get ATA address for currencyMint + farmsPda (initialized)
    const splAtaDestination = await getAssociatedTokenAddress(currencyMint, farmsPda, true);

    // Get ATA address for fieldNft mint + farmsPda (initialized)
    const fieldAtaSource = await getAssociatedTokenAddress(fieldNft.mint.address, farmsPda, true);
    // Get ATA address for fieldNft mint + userFarmPda (*un*initialized)
    const fieldAtaDestination = await getAssociatedTokenAddress(fieldNft.mint.address, userWallet.publicKey);

    
    const [trifleAddress] = findTriflePda(fieldNft.mint.address, farmsPda); // Important!!!! Has to be derived from farmsPda
    const [escrowAccountAddress] = findEscrowPda(fieldNft.mint.address, 1, trifleAddress);

    console.log("Buys new field and creates trifle account");

    const buyNewFieldSignature = await program.methods.buyField()
      .accounts({
        farmsPda,
        userAccount: userWallet.publicKey,
        splAtaSource,
        splAtaDestination,
        splMint: currencyMint,
        fieldAtaSource,
        fieldAtaDestination,
        fieldMint: fieldNft.mint.address,
        farmerHouseProgram: program.programId,
        escrowAccount: escrowAccountAddress,
        escrowConstraintModel: escrowConstraintModelAddress,
        trifleAccount: trifleAddress,
        fieldMetadata: fieldNft.metadataAddress,
        fieldMasterEdition: fieldNft.edition.address,
        tokenMetadataProgram: metadata.PROGRAM_ID,
        trifleProgram: trifle.PROGRAM_ID,
        instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .signers([userWallet])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: buyNewFieldSignature,
    });

    console.log("Field bought");
  })

  it("Buys Seed", async () => {
    // Get ATA address for currencyMint + userWallet (initialized)
    const splAtaSource = await getAssociatedTokenAddress(currencyMint, userWallet.publicKey, false);
    // Get ATA address for currencyMint + farmsPda (initialized)
    const splAtaDestination = await getAssociatedTokenAddress(currencyMint, farmsPda, true);

    // Get ATA address for seedNft mint + farmsPda (initialized)
    const seedAtaSource = await getAssociatedTokenAddress(tomatoNft.mint.address, farmsPda, true);
    // Get ATA address for seedNft mint + userFarmPda (*un*initialized)
    const seedAtaDestination = await getAssociatedTokenAddress(tomatoNft.mint.address, userWallet.publicKey, true);

    console.log("Buying new seed");

    const buyNewSeedSignature = await program.methods.buySeed()
      .accounts({
        farmsPda,
        userAccount: userWallet.publicKey,
        splAtaSource,
        splAtaDestination,
        splMint: currencyMint,
        seedAtaSource,
        seedAtaDestination,
        seedMint: tomatoNft.mint.address,
        seedMetadata: tomatoNft.metadataAddress,
        farmerHouseProgram: program.programId
      })
      .signers([userWallet])
      .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 }), anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 0 })])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: buyNewSeedSignature,
    });

    console.log("Seed bought");
  })

  it("Plants seed", async () => {

    const [trifleAddress] = findTriflePda(fieldNft.mint.address, farmsPda);
    const [escrowAccountAddress] = findEscrowPda(fieldNft.mint.address, 1, trifleAddress);
    const fieldTokenAccount = await getAssociatedTokenAddress(fieldNft.mint.address, userWallet.publicKey, true);
    const attributeSrcTokenAccount = await getAssociatedTokenAddress(tomatoNft.mint.address, userWallet.publicKey, true)
    const attributeDstTokenAccount = await getAssociatedTokenAddress(tomatoNft.mint.address, escrowAccountAddress, true);

    const plantSeedSignature = await program.methods.plantSeed("a1")
      .accounts({
        trifleAccount: trifleAddress,
        farmsPda,
        userAccount: userWallet.publicKey,
        escrowConstraintModel: escrowConstraintModelAddress,
        escrowAccount: escrowAccountAddress,
        fieldMint: fieldNft.mint.address,
        fieldTokenAccount,
        fieldMasterEdition: fieldNft.edition.address,
        attributeMint: tomatoNft.mint.address,
        attributeSrcTokenAccount,
        attributeDstTokenAccount,
        attributeMetadata: tomatoNft.metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenMetadataProgram: metadata.PROGRAM_ID,
        farmerHouseProgram: program.programId,
        trifleProgram: trifle.PROGRAM_ID,
      })
      .signers([userWallet])
      .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 }), anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 0 })])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: plantSeedSignature,
    });

    metaplex.nfts().unverifyCollection({
      mintAddress: tomatoNft.mint.address,
      collectionMintAddress: tomatoSeedCollection.mint.address,
    })

  })

  it("Waters seed twice", async () => {

    const fieldAssociatedToken = await getAssociatedTokenAddress(fieldNft.mint.address, userWallet.publicKey);
    const [trifleAddress] = findTriflePda(fieldNft.mint.address, farmsPda);

    console.log("Watering crop for the first time")
    const firstWaterSignature = await program.methods.water("a1")
      .accounts({
        farmsPda,
        userAccount: userWallet.publicKey,
        fieldMint: fieldNft.mint.address,
        fieldAssociatedToken,
        cropMetadata: tomatoNft.metadataAddress,
        trifleAccount: trifleAddress,
        tokenMetadataProgram: metadata.PROGRAM_ID,
        farmerHouseProgram: program.programId,
      })
      .signers([userWallet])
      .rpc()

    const firstWaterBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: firstWaterBlockHash.blockhash,
      lastValidBlockHeight: firstWaterBlockHash.lastValidBlockHeight,
      signature: firstWaterSignature,
    });
    console.log("Crop watered one time");

    console.log("Watering crop for the second time")
    const secondWaterSignature = await program.methods.water("a1")
      .accounts({
        farmsPda,
        userAccount: userWallet.publicKey,
        fieldMint: fieldNft.mint.address,
        fieldAssociatedToken,
        cropMetadata: tomatoNft.metadataAddress,
        trifleAccount: trifleAddress,
        tokenMetadataProgram: metadata.PROGRAM_ID,
        farmerHouseProgram: program.programId,
      })
      .signers([userWallet])
      .rpc()

    const secondWaterBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: secondWaterBlockHash.blockhash,
      lastValidBlockHeight: secondWaterBlockHash.lastValidBlockHeight,
      signature: secondWaterSignature,
    });
    console.log("Crop watered twice");

    await metaplex.nfts().findByMint({ mintAddress: tomatoNft.mint.address }).then(data =>
      assert.equal(data.collection.address.toBase58(), ripeTomatoCollection.mint.address.toBase58())
    );
  })

  it("Harvests crop", async () => {

    const [trifleAddress] = findTriflePda(fieldNft.mint.address, farmsPda);
    const [escrowAccountAddress] = findEscrowPda(fieldNft.mint.address, 1, trifleAddress);
    const fieldTokenAccount = await getAssociatedTokenAddress(fieldNft.mint.address, userWallet.publicKey, true);
    const attributeSrcTokenAccount = await getAssociatedTokenAddress(tomatoNft.mint.address, escrowAccountAddress, true)
    const attributeDstTokenAccount = await getAssociatedTokenAddress(tomatoNft.mint.address, userWallet.publicKey);

    const plantSeedSignature = await program.methods.harvestCrop("a1")
      .accounts({
        trifleAccount: trifleAddress,
        farmsPda,
        userAccount: userWallet.publicKey,
        escrowConstraintModel: escrowConstraintModelAddress,
        escrowAccount: escrowAccountAddress,
        fieldMint: fieldNft.mint.address,
        fieldTokenAccount,
        fieldEdition: fieldNft.edition.address,
        fieldMetadata: fieldNft.metadataAddress,
        attributeMint: tomatoNft.mint.address,
        attributeSrcTokenAccount,
        attributeDstTokenAccount,
        attributeMetadata: tomatoNft.metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenMetadataProgram: metadata.PROGRAM_ID,
        farmerHouseProgram: program.programId,
        trifleProgram: trifle.PROGRAM_ID,
        instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .signers([userWallet])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: plantSeedSignature,
    });

  })

  it("Buys Seed", async () => {
    // Get ATA address for currencyMint + userWallet (initialized)
    const splAtaSource = await getAssociatedTokenAddress(currencyMint, farmsPda, true);
    // Get ATA address for currencyMint + farmsPda (initialized)
    const splAtaDestination = await getAssociatedTokenAddress(currencyMint, userWallet.publicKey);

    // Get ATA address for seedNft mint + farmsPda (initialized)
    const cropAtaSource = await getAssociatedTokenAddress(tomatoNft.mint.address, userWallet.publicKey);
    // Get ATA address for seedNft mint + userFarmPda (*un*initialized)
    const cropAtaDestination = await getAssociatedTokenAddress(tomatoNft.mint.address, farmsPda, true);

    console.log("Selling crop");

    const sellCropSignature = await program.methods.sellCrop()
      .accounts({
        farmsPda,
        userAccount: userWallet.publicKey,
        splAtaSource,
        splAtaDestination,
        splMint: currencyMint,
        cropAtaSource,
        cropAtaDestination,
        cropMetadata: tomatoNft.metadataAddress,
        farmerHouseProgram: program.programId
      })
      .signers([userWallet])
      .rpc();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sellCropSignature,
    });

    console.log("Crop Sold");
  })

});