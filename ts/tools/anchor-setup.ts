import * as anchor from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

// Import your ru
const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

const connection = anchor.getProvider().connection;

const signer = anchor.web3.Keypair.generate();

const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), signer.publicKey.toBuffer()], program.programId)[0];

const confirm = async (signature: string): Promise<string> => {
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature,
    ...block
  })
  return signature
}

  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  }

  it("Airdrop", async () => {
    await connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log)
  })