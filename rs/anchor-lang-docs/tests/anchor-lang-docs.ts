import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorLangDocs } from "../target/types/anchor_lang_docs";
import { expect } from "chai";

describe("anchor-lang-docs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorLangDocs as Program<AnchorLangDocs>;
  const myAccount = anchor.web3.Keypair.generate();
  const tokenAccount = anchor.web3.Keypair.generate();
  const signer = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.

    const tx = await program.methods
    .initialize({
      data: new anchor.BN(5),
      age: new anchor.BN(1)
    })
    .accounts({
      myAccount: myAccount.publicKey,
      owner: signer.publicKey,
    })
    .signers([signer])
    .rpc();

    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    expect(account.data).to.equal("MyAccount");
  });

  it("Set Data", async () => {
    // Add your test here.

    const tx = await program.methods
    .setData({
      data: new anchor.BN(5),
      age: new anchor.BN(1)
    })
    .accounts({
      myAccount: myAccount.publicKey,
      owner: signer.publicKey,
    })
    .signers([signer])
    .rpc();

    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    expect(account.data).to.equal("MyAccount");
  });




});
