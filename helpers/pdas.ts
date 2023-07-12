import { PublicKey } from '@solana/web3.js';
import { EscrowAuthority, PROGRAM_ADDRESS as TM_PROGRAM_ADDRESS } from '@metaplex-foundation/mpl-token-metadata';
import { PROGRAM_ADDRESS as TRIFLE_PROGRAM_ADDRESS } from '@metaplex-foundation/mpl-trifle';

export const findEscrowConstraintModelPda = (creator: PublicKey, name: String) => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from('escrow'),
            creator.toBuffer(),
            Buffer.from(name),
        ],
        new PublicKey(TRIFLE_PROGRAM_ADDRESS)
    );
}

export const findTriflePda = (mint: PublicKey, authority: PublicKey) => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from('trifle'),
            mint.toBuffer(),
            authority.toBuffer(),
        ],
        new PublicKey(TRIFLE_PROGRAM_ADDRESS)
    );
}

export const findEscrowPda = (mint: PublicKey, authority: number, creator?: PublicKey) => {
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