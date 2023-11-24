import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    Data,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { fromWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";
import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

//Create a devnet connection
const umi = createUmi(connection);
const umiKeypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, umiKeypair);
umi.use(signerIdentity(signer, umiKeypair)));


// Define our Mint address
const mint = new PublicKey("8VfXmNLMuJ6vSmZmXkS8gtJi8j1TwZ3TR6cjTFyVZSf6")

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from('metadata'),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(metadata_seeds, token_metadata_program_id);

(async () => {
    try {
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer
        }
        let data: DataV2Args = {
            name: "Test",
            symbol: "TST",
            uri: "",
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null
        }
        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null
                
        }
        let tx = createMetadataAccountV3(
            umi, {
                ...accounts,
                ...args
            }
        )

        let results = await tx.sendAndConfirm(umi).then(r => base58.encode(r.signature));
        console.log(`Your Metadata account was created: https://explorer.solana`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();