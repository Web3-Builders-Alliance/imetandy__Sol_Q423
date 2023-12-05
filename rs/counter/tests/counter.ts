import * as anchor from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("Counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as anchor.Program<Counter>;
  const counter = anchor.web3.Keypair.generate();
  const provider = anchor.getProvider();
  const signer = anchor.web3.Keypair.generate();

  let countValue = 0;

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    });
    return signature;
  }

  it("Airdrop 1 SOL!", async () => {
    const tx = await provider.connection.requestAirdrop(counter.publicKey, 1e9).then(confirm);
  });
  it("Initialize", async () => {
    const tx = await program.methods.initialize(new anchor.BN(countValue)).accounts({
      signer: signer.publicKey,
      counter: counter.publicKey,
      systemProgram: program.programId,
    });    
  });  

  it("Add x to counter", async () => {
    const tx = await program.methods.increment(new anchor.BN(5));
    const account = await program.account.counter.fetch(counter.publicKey);
    console.log(account);
  });

  it("Remove x from counter", async () => {
    const tx = await program.methods.decrement(new anchor.BN(5));
    let countValue = (await program.account.counter.fetch(counter.publicKey)).count.toNumber();
    console.log(countValue);
    ;
  });


});
