import * as anchor from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

describe("Counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as anchor.Program<Counter>;
  
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const signer = anchor.web3.Keypair.generate();
  const counter = findProgramAddressSync([Buffer.from("counter"), signer.publicKey.toBytes()] , program.programId)[0];
  
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

  it("Airdrop Tokens", async () => {
    const tx1 = await provider.connection.requestAirdrop(
      signer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    await provider.connection.confirmTransaction(tx1);
    });

  it("Initialize", async () => {
    const tx = await program.methods.initialize()
    .accounts({
      signer: signer.publicKey,
      counter: counter,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc().then(confirm).then(log);    
  });  

  it("Add x to counter", async () => {
    const tx = await program.methods.increment(new anchor.BN(5))
    .accounts({
      counter: counter,
      signer: signer.publicKey,
      }).signers([signer]).rpc().then(confirm).then(log);
    const account = (await program.account.counter.fetch(counter)).count;
    console.log(account.toNumber());
  });

  it("Remove x from counter", async () => {
    const tx = await program.methods.decrement(new anchor.BN(5))
    .accounts({
      signer: signer.publicKey,
        counter: counter,
        }).signers([signer]).rpc().then(confirm).then(log);
    let countValue = (await program.account.counter.fetch(counter)).count;
    console.log(countValue.toNumber());
    ;
  });

});

