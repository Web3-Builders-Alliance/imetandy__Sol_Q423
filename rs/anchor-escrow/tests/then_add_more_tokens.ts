import * as anchor from "@coral-xyz/anchor";
import { Program} from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint, MINT_SIZE, createInitializeMint2Instruction, createAssociatedTokenAccountIdempotentInstruction, createMintToInstruction} from "@solana/spl-token"
import { randomBytes } from "crypto";
import { SystemProgram} from "@solana/web3.js";


describe("add more tokens", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const seed = new anchor.BN(randomBytes(8));
  // Create maker and taker
  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();
  // Create mints
  const mint_a = anchor.web3.Keypair.generate();
  const mint_b = anchor.web3.Keypair.generate();

  const maker_ata_a = getAssociatedTokenAddressSync(mint_a.publicKey, maker.publicKey);
  const maker_ata_b = getAssociatedTokenAddressSync(mint_b.publicKey, maker.publicKey);
  const taker_ata_a = getAssociatedTokenAddressSync(mint_a.publicKey, taker.publicKey);
  const taker_ata_b = getAssociatedTokenAddressSync(mint_b.publicKey, taker.publicKey);
      
  //  escrow should have buffer from escrow, maker public key, and seed
  // in rust, seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()] 
  const escrow = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow"), 
      maker.publicKey.toBuffer(),
      seed.toBuffer('le', 8)  
    ],
    program.programId)[0];

  //  vault
const vault = getAssociatedTokenAddressSync(mint_a.publicKey, escrow, true);

// Helper Scripts
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

  //Airdrop tokens to both accounts
  it("Airdrop Tokens to Maker and Taker", async () => {
    const tx_maker = await provider.connection.requestAirdrop(
      maker.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    await provider.connection.confirmTransaction(tx_maker);
    const tx_taker = await provider.connection.requestAirdrop(
      taker.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*10
    );
    await provider.connection.confirmTransaction(tx_taker);
    console.log(`Maker airdrop tx: ${tx_maker}`);
    console.log(`Maker airdrop tx: ${tx_taker}`);
  });


  it("Mint Tokens to accounts", async () => {
    let lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new anchor.web3.Transaction();
    tx.instructions =[
      // Create the mint accounts
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey, 
        newAccountPubkey: mint_a.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID
        }),
        SystemProgram.createAccount({
        fromPubkey: provider.publicKey, 
        newAccountPubkey: mint_b.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID
        }),
    createInitializeMint2Instruction(
      mint_a.publicKey,     // The mint
      6,                    // Decemal places
      maker.publicKey,      // Authority
      null,      
    ),
    createInitializeMint2Instruction(
      mint_b.publicKey,     // The mint
      6,                    // Decemal places
      taker.publicKey,      // Authority
      null,               // Freeze Authority
    ),
    // Create the associated token accounts
    createAssociatedTokenAccountIdempotentInstruction(
      maker.publicKey,  // Payer
      maker_ata_a,      // associated token account
      maker.publicKey,  // owner
      mint_a.publicKey  // mint
    ),
    createAssociatedTokenAccountIdempotentInstruction(
      taker.publicKey,  // Payer
      taker_ata_b,      // associated token account
      taker.publicKey,  // owner
      mint_b.publicKey  // mint
    ),
    createMintToInstruction(
        mint_a.publicKey, // mint
        maker_ata_a,
        maker.publicKey,
        20e9
    ),
    createMintToInstruction(
      mint_b.publicKey, // mint
      taker_ata_b,
      taker.publicKey,
      100e9
  ),
]
await provider.sendAndConfirm(
  tx, [mint_a, mint_b, maker, taker], {commitment: 'confirmed'}).then(log);
  console.log(maker_ata_a)
});

  xit("Make", async () => {
    // Add your test here.
    const tx = await program.methods.make(  
      seed,
      new anchor.BN(1e6),
      new anchor.BN(1e6),
    ).accounts({
      maker: maker.publicKey,
      mintA: mint_a.publicKey,
      mintB: mint_b.publicKey,
      makerAtaA: maker_ata_a,
      escrow,
      vault,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([maker]).rpc().then(confirm).then(log);

    console.log("Your transaction signature", tx);
  });

  xit("Take", async () => {
    // Add your test here.
    const tx = await program.methods.take(
    ).accounts(
      {
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: mint_a.publicKey,
        mintB: mint_b.publicKey,
        takerAtaA: taker_ata_a,
        takerAtaB: taker_ata_b,
        makerAtaB: maker_ata_b,
        escrow,
        vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      }
    ).signers([taker]).rpc().then(confirm).then(log);
    console.log("Your transaction signature", tx);
  });

  xit("Refund", async () => {
    // Add your test here.
    const tx = await program.methods.refund().rpc();
    console.log("Your transaction signature", tx);
  });





});
