import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
    try {
        const image = "https://arweave.net/9eXB70STe3evQ-TInp1i5W8CEJrcZBglq0nqkAQ9w1w"
        const metadata = 
        {
            "name": "Generug - Movie",
            "symbol": "RUG",
            "description": "A rare and exotic rug that looks like a movie reel",
            "seller_fee_basis_points": 100,
            image,
            "attributes": [
              {
                trait_type: "background",
                value: "Grey"
              },
              {
                trait_type: "maincolor",
                value: "Purple"
             },
             {
                "trait_type": "highlights",
                "value": "Multicolor"
              }
            ],
            "properties": {
              "files": [
                {
                    type: "image/png",
                    uri: image
                }
              ],
              "creators": [
                {
                  "address": keypair.publicKey,
                  "share": 100
                }
              ]
            }
          }
          const myUri = await bundlrUploader.uploadJson(metadata);
          console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();