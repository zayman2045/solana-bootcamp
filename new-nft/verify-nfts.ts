import {
  findMetadataPda,
  mplTokenMetadata,
  verifyCollectionV1,
} from "@metaplex-foundation/mpl-token-metadata";

import {
  airdropIfRequired,
  getExplorerLink,
  getKeypairFromFile,
} from "@solana-developers/helpers";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import { Connection, LAMPORTS_PER_SOL, clusterApiUrl } from "@solana/web3.js";

import {
  keypairIdentity,
  publicKey,
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

// Import the collection address
const collectionAddress = publicKey(
  "ddiLQf9dppnYAwEXvypcGazVFnYLJCWWe9WVNZfcd6u"
);

// Import the NFT address
const nftAddress = publicKey("gAtaYvnfDGA5yFn6P4ZLDu9Saxp6bpxXhbYGbUdLqc4");

const transaction = await verifyCollectionV1(umi, {
  metadata: findMetadataPda(umi, { mint: nftAddress }),
  collectionMint: collectionAddress,
  authority: umi.identity,
});

// Send and confirm the transaction
transaction.sendAndConfirm(umi);

console.log(
  `NFT ${nftAddress} verified in collection ${collectionAddress}! 🎉 See it on the blockchain: ${getExplorerLink(
    "transaction",
    nftAddress,
    "devnet"
  )}`
);
