import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("8VfXmNLMuJ6vSmZmXkS8gtJi8j1TwZ3TR6cjTFyVZSf6");
// Recipient address
const to = new PublicKey("HDaVYzEeTsu5v3YzojroWt3GLAhHNvDja6VnCjURwJHk");

(async () => {
    try {
        // Create a fromATA
        const fromAta = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            keypair.publicKey
            )
        console.log(`Your ata is: ${fromAta.address.toBase58()}`);
        // Create a toATA
        const toAta = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            to
            )
        console.log(`Your toAta is: ${toAta.address.toBase58()}`);
        
        let sig = await transfer(
            connection, 
            keypair, 
            fromAta.address, 
            toAta.address, 
            keypair, 
            1_000_000 );
        console.log(`${sig}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();