import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;
  
  const provider = anchor.getProvider();

  const signer = anchor.web3.Keypair.generate();

  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), signer.publicKey.toBuffer()], program.programId)[0];

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    })
    return signature
  }

  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    return signature;
  }
  it("Airdrop Tokens", async () => {
    const tx = await provider.connection.requestAirdrop(
      signer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    await provider.connection.confirmTransaction(tx);
  }
  );
  it("Deposit 5 SOL into Vault!", async () => {

     const tx = await program.methods
     .deposit(new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL))
     .accounts({
      signer: signer.publicKey,
      vault: vault,
      systemProgram: anchor.web3.SystemProgram.programId
     })
      .signers([signer])
      .rpc()
      .then(confirm)
      .then(log)
    }
  );
  it("Withdraw 5 SOL into Vault!", async () => {

    const tx = await program.methods
    .close()
    .accounts({
     signer: signer.publicKey,
     vault: vault,
     systemProgram: anchor.web3.SystemProgram.programId
    })
     .signers([signer])
     .rpc()
     .then(confirm)
     .then(log)
   }
 );

});
function findProgramAddress(arg0: Buffer[], programId: anchor.web3.PublicKey) {
  throw new Error("Function not implemented.");
}

