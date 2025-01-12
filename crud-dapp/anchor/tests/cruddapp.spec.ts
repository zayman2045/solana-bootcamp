import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Crud } from "anchor/target/types/crud";

describe("Crud", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const crudProgram = anchor.workspace.Crud as Program<Crud>;

  it("Create entry", async () => {
    await crudProgram.methods.createEntry("My Title", "My Message").rpc();

    const [entryAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("My Title"), payer.publicKey.toBytes()],
      crudProgram.programId
    );

    const entryAccount = await crudProgram.account.entry.fetch(entryAddress);

    expect(entryAccount.message).toEqual("My Message");
    expect(entryAccount.title).toEqual("My Title");
  });

  it("Delete entry", async () => {
    const result = await crudProgram.methods.deleteEntry("My Title").rpc();
    console.log(result);
  });
});
