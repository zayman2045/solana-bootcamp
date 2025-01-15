import {
  createNft,
  fetchDigitalAsset,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";

import {
  airdropIfRequired,
  getExplorerLink,
  getKeypairFromFile,
} from "@solana-developers/helpers";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import { Connection, LAMPORTS_PER_SOL, clusterApiUrl } from "@solana/web3.js";

import {
  generateSigner,
  keypairIdentity,
  percentAmount,
} from "@metaplex-foundation/umi";

// Create a connection to the cluster
const connection = new Connection(clusterApiUrl("devnet"));

// Load the user keypair (if empty, loads from the default path)
const user = await getKeypairFromFile();

// Request airdrop if required
await airdropIfRequired(
  connection,
  user.publicKey,
  1 * LAMPORTS_PER_SOL,
  0.5 * LAMPORTS_PER_SOL
);

// Log the user public key
console.log("Loaded user", user.publicKey.toBase58());

// Create Umi instance to interact with the Metaplex API
const umi = createUmi(connection.rpcEndpoint);
umi.use(mplTokenMetadata());

// Set up Umi instance to use the user keypair for signing
const umiUser = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
umi.use(keypairIdentity(umiUser));

console.log("Set up Umi instance for user");

// Generate a keypair for the collection mint
const collectionMint = generateSigner(umi);

// Create the collection transaction
const transaction = await createNft(umi, {
  mint: collectionMint,
  name: "My Collection",
  symbol: "MC",
  uri: "https://...",
  sellerFeeBasisPoints: percentAmount(0),
  isCollection: true,
});

// Send and confirm the transaction
await transaction.sendAndConfirm(umi);

// Fetch the created collection
const createdCollectionNft = await fetchDigitalAsset(
  umi,
  collectionMint.publicKey
);

console.log(
  `Created Collection 📦! Address: ${getExplorerLink(
    "address",
    createdCollectionNft.mint.publicKey,
    "devnet"
  )}`
);
