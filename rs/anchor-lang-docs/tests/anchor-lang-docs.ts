import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorLangDocs } from "../target/types/anchor_lang_docs";

describe("anchor-lang-docs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const myAccount = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();

  const program = anchor.workspace.AnchorLangDocs as Program<AnchorLangDocs>;
  it("Airdrop Tokens", async () => {
    const tx1 = await provider.connection.requestAirdrop(
      payer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    const tx2 = await provider.connection.requestAirdrop(
      myAccount.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*5
    );
    await provider.connection.confirmTransaction(tx1);
    await provider.connection.confirmTransaction(tx2);
    
    // log payer sol amount
    const payerBalance = await provider.connection.getBalance(payer.publicKey);
    const myAccountBalance = await provider.connection.getBalance(myAccount.publicKey);
    console.log("Payer balance:", payerBalance);
    console.log("My account balance:", myAccountBalance);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize()
    .accounts({
        myAccount: myAccount.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([payer]).rpc();

    // log the new data value on my account
    const myAccountData = (await program.account.myAccount.fetch(myAccount.publicKey)).data;
    console.log("My account data:", myAccountData.toString());
    console.log("Your transaction signature", tx);
    });


    // Updating the data 
    xit("Update data!", async () => {
      const tx = await program.methods.setData(new anchor.BN(100)).accounts(
        {
          myAccount: myAccount.publicKey,
          payer: payer.publicKey,
        }
      ).signers([payer]).rpc();
      const myAccountData = (await program.account.myAccount.fetch(myAccount.publicKey)).data;
      console.log("My account data:", myAccountData.toString());
      console.log("Your transaction signature", tx);

  });
});