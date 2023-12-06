import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "../programs/wba_vault";
import wallet from "../wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create a random keypair
const vaultState = new PublicKey("4St9WLA6LgsxJ3u2Jb4Uwi3yZkCbGQLZC4gKnY8fFiZh");

// 77FQ9XNx7e14fJJGNs3j8KjeJ36gW98EoVddp22bCsDC
// Create a random keypair
const closeVaultState = new PublicKey("4St9WLA6LgsxJ3u2Jb4Uwi3yZkCbGQLZC4gKnY8fFiZh");

(async () => {
  try {
    const signature = await program.methods
    .closeAccount()
    .accounts({
      owner: keypair.publicKey,
      vaultState: vaultState,
      closeVaultState: closeVaultState,
      systemProgram: SystemProgram.programId
        })
    .signers([
    keypair
    ]).rpc();
    console.log(`Close success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
