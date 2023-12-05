import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  const provider = anchor.getProvider();

  const signer = anchor.web3.Keypair.generate();

  const userCounter = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter"), signer.publicKey.toBuffer()], program.programId)[0];

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    });
    return signature;
  }

  it("Airdrop 1 SOL!", async () => {
    const tx = await provider.connection.requestAirdrop(signer.publicKey, 1e9).then(confirm);
    console.log(program.account.counter);
  });
  
  it("Add 1 to counter", async () => {
    const tx = await program.methods.increment();
  });


});
