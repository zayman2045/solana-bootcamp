import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Voting } from "../target/types/voting";
import { BankrunProvider, startAnchor } from "anchor-bankrun";

const IDL = require("../target/idl/voting.json");

const votingAddress = new PublicKey(
  "2Q11KevbxUyKxMYCk5yQHG8yizMPwZFwS1BAzofrAAFo"
);

describe("Voting", () => {
  let context;
  let provider;
  anchor.setProvider(anchor.AnchorProvider.env());
  let votingProgram = anchor.workspace.Voting as Program<Voting>;

  beforeAll(async () => {
    // context = await startAnchor(
    //   "",
    //   [{ name: "voting", programId: votingAddress }],
    //   []
    // );
    // provider = new BankrunProvider(context);
    // votingProgram = new Program<Voting>(IDL, provider);
  });

  it("Initialize poll", async () => {
    await votingProgram.methods
      .initializePoll(
        new anchor.BN(1),
        "Who is your favorite basketball player?",
        new anchor.BN(0),
        new anchor.BN(1836469290)
      )
      .rpc();

    const [pollAddress] = await PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      votingAddress
    );

    const poll = await votingProgram.account.poll.fetch(pollAddress);

    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.description).toEqual("Who is your favorite basketball player?");
    expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());
  });

  it("Initialize candidate", async () => {
    await votingProgram.methods
      .initializeCandidate("LeBron James", new anchor.BN(1))
      .rpc();

    await votingProgram.methods
      .initializeCandidate("Stephen Curry", new anchor.BN(1))
      .rpc();

    const [lebronAddress] = await PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
        Buffer.from("LeBron James"),
      ],
      votingAddress
    );

    const lebronAccount = await votingProgram.account.candidate.fetch(
      lebronAddress
    );

    expect(lebronAccount.candidateVotes.toNumber()).toEqual(0);
    expect(lebronAccount.candidateName).toEqual("LeBron James");

    const [curryAddress] = await PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
        Buffer.from("Stephen Curry"),
      ],
      votingAddress
    );

    const curryAccount = await votingProgram.account.candidate.fetch(
      curryAddress
    );

    expect(curryAccount.candidateName).toEqual("Stephen Curry");
    expect(curryAccount.candidateVotes.toNumber()).toEqual(0);
  });

  it("Vote", async () => {
    await votingProgram.methods.vote("LeBron James", new anchor.BN(1)).rpc();

    const [lebronAddress] = PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
        Buffer.from("LeBron James"),
      ],
      votingAddress
    );

    const lebronAccount = await votingProgram.account.candidate.fetch(
      lebronAddress
    );

    expect(lebronAccount.candidateVotes.toNumber()).toEqual(1);
  });
});
