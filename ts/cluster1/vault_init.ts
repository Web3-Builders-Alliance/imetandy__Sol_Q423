import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";
import { WbaVault, IDL } from "../programs/wba_vault";
import wallet from "../wba-wallet.json";

/// J8qKEmQpadFeBuXAVseH8GNrvsyBhMT8MHSVD3enRgJz
// decode the base58 private key to a buffer
const buffer = wallet;

// convert the buffer to a Uint8Array
const uint8ArrayPrivateKey = new Uint8Array(wallet);
const keypair = Keypair.fromSecretKey(new Uint8Array(uint8ArrayPrivateKey));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider,
);

// Create a random keypair
const vaultState = Keypair.generate();
console.log(`Vault public key: ${vaultState.publicKey.toBase58()}`);

// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
const vaultAuthKeys = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
const vaultAuth = PublicKey.findProgramAddressSync(vaultAuthKeys, program.programId)[0];

// Create the vault key
// Seeds are "vault", vaultAuth
const vaultKeys = [Buffer.from("vault"), vaultAuth.toBuffer()];
const vault = PublicKey.findProgramAddressSync(vaultKeys, program.programId)[0];

// Execute our enrollment transaction
(async () => {
  try {
    const signature = await program.methods.initialize()
    .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuth,
        vault: vault,
        systemProgram: SystemProgram.programId

    }).signers([keypair, vaultState]).rpc();
    console.log(`Init success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
