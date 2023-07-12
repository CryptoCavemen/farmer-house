import { NftWithToken, SftWithToken, Nft } from "@metaplex-foundation/js";
import { Connection, Keypair, PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, Transaction } from "@solana/web3.js";
import { createCreateTrifleAccountInstruction, createAddNoneConstraintToEscrowConstraintModelInstruction, createCreateEscrowConstraintModelAccountInstruction, createTransferInInstruction, EscrowConstraintType } from "@metaplex-foundation/mpl-trifle";
import { findEscrowPda, findTriflePda, findEscrowConstraintModelPda } from "./pdas";
import { getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { EscrowAuthority, PROGRAM_ADDRESS as TM_PROGRAM_ADDRESS } from "@metaplex-foundation/mpl-token-metadata";

export const createConstraintModel = async (connection: Connection, keypair: Keypair) => {
    let escrowConstraintModel = findEscrowConstraintModelPda(keypair.publicKey, "test");

    let createIX = createCreateEscrowConstraintModelAccountInstruction(
        {
            escrowConstraintModel: escrowConstraintModel[0],
            payer: keypair.publicKey,
            updateAuthority: keypair.publicKey,
        },
        {
            createEscrowConstraintModelAccountArgs: {
                name: "test",
                schemaUri: "https://shdw-drive.genesysgo.net/G6yhKwkApJr1YCCmrusFibbsvrXZa4Q3GRThSHFiRJQW/schema.json"
            }
        }
    );

    let addIXes = [
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a1",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a2",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a3",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a4",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a5",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "a6",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b1",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b2",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b3",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b4",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b5",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "b6",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c1",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c2",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c3",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c4",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c5",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
        createAddNoneConstraintToEscrowConstraintModelInstruction(
            {
                constraintModel: escrowConstraintModel[0],
                payer: keypair.publicKey,
                updateAuthority: keypair.publicKey,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            {
                addNoneConstraintToEscrowConstraintModelArgs: {
                    constraintName: "c6",
                    tokenLimit: 1,
                    transferEffects: 1,
                }
            }
        ),
    ];

    let tx = new Transaction().add(createIX, ...addIXes);

    const { blockhash } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = keypair.publicKey;
    await connection.sendTransaction(tx, [keypair], { skipPreflight: true });

    return escrowConstraintModel[0];
}

export const createTrifle = async (connection: Connection, nft: Nft, keypair: Keypair) => {
    let escrowConstraintModel = findEscrowConstraintModelPda(keypair.publicKey, "test");
    let trifleAddress = await findTriflePda(nft.mint.address, keypair.publicKey);
    let escrowAccountAddress = await findEscrowPda(nft.mint.address, 1, trifleAddress[0]);
    let tokenAccount = await getAssociatedTokenAddress(nft.mint.address, keypair.publicKey);

    let createIX = createCreateTrifleAccountInstruction(
        {
            escrow: escrowAccountAddress[0],
            metadata: nft.metadataAddress,
            mint: nft.mint.address,
            tokenAccount: tokenAccount,
            edition: nft.edition.address,
            trifleAccount: trifleAddress[0],
            trifleAuthority: keypair.publicKey,
            constraintModel: escrowConstraintModel[0],
            payer: keypair.publicKey,
            tokenMetadataProgram: new PublicKey(TM_PROGRAM_ADDRESS),
            sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        },
    );

    let tx = new Transaction().add(createIX);

    const { blockhash } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = keypair.publicKey;
    await connection.sendTransaction(tx, [keypair], { skipPreflight: true });

    return escrowAccountAddress[0];
}

export const transferIn = async (connection: Connection, escrowNft: Nft, escrowAccountAddress: PublicKey, nft: NftWithToken | SftWithToken, keypair: Keypair, slot: string) => {
    let escrowConstraintModel = findEscrowConstraintModelPda(keypair.publicKey, "test");
    let trifleAddress = await findTriflePda(escrowNft.mint.address, keypair.publicKey);
    let tokenAccount = await getAssociatedTokenAddress(escrowNft.mint.address, keypair.publicKey);

    let dst: PublicKey = await getAssociatedTokenAddress(nft.mint.address, escrowAccountAddress, true);
    let transferIX = createTransferInInstruction(
        {
            trifle: trifleAddress[0],
            trifleAuthority: keypair.publicKey,
            payer: keypair.publicKey,
            constraintModel: escrowConstraintModel[0],
            escrow: escrowAccountAddress,
            attributeMint: nft.mint.address,
            attributeSrcToken: nft.token.address,
            attributeDstToken: dst,
            attributeMetadata: nft.metadataAddress,
            escrowMint: escrowNft.mint.address,
            escrowToken: tokenAccount,
            splToken: new PublicKey(TOKEN_PROGRAM_ID),
            splAssociatedTokenAccount: new PublicKey(ASSOCIATED_TOKEN_PROGRAM_ID),
            tokenMetadataProgram: new PublicKey(TM_PROGRAM_ADDRESS),
        },
        {
            transferInArgs: { amount: 1, slot }
        }
    );

    let tx = new Transaction().add(transferIX);

    // let accountInfo = await connection.getAccountInfo(nft.token.address);
    // if (accountInfo){
    //     let account = AccountLayout.decode(accountInfo.data);
    //     console.log(account);
    // }

    const { blockhash } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = keypair.publicKey;
    // console.log(tx);
    await connection.sendTransaction(tx, [keypair], { skipPreflight: true });
}