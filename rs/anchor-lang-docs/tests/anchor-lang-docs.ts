import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorLangDocs } from "../target/types/anchor_lang_docs";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";


describe("anchor-lang-docs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.AnchorLangDocs as Program<AnchorLangDocs>;
  
  const provider = anchor.getProvider();
  const payer = anchor.web3.Keypair.generate();
  const myAccount = findProgramAddressSync([Buffer.from("my_account"), payer.publicKey.toBytes()] , program.programId)[0];

  it("Airdrop Tokens", async () => {
    const tx1 = await provider.connection.requestAirdrop(
      payer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    await provider.connection.confirmTransaction(tx1);

    // log payer sol amount
    const payerBalance = await provider.connection.getBalance(payer.publicKey);
    console.log("Payer balance:", payerBalance);
     });

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize()
    .accounts({
        myAccount: myAccount,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([payer]).rpc();

    // log the new data value on my account
    const myAccountData = (await program.account.myAccount.fetch(myAccount)).data;
    console.log("My account data:", myAccountData.toString());
    console.log("Your transaction signature", tx);
    });


    // Updating the data 
    it("Update data!", async () => {
      const tx = await program.methods.setData(new anchor.BN(100)).accounts(
        {
          myAccount: myAccount,
          payer: payer.publicKey,
        }
      ).signers([payer]).rpc();
      const myAccountData = (await program.account.myAccount.fetch(myAccount)).data;
      console.log("My account data:", myAccountData.toString());
      console.log("Your transaction signature", tx);

  });
});


