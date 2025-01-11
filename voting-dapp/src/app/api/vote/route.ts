import { Program, BN } from "@coral-xyz/anchor";
import {
  ActionGetResponse,
  ActionPostRequest,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
} from "@solana/actions";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import { Voting } from "anchor/target/types/voting";

const IDL = require("anchor/target/idl/voting.json");

export const OPTIONS = GET;

export async function GET(request: Request) {
  const actionMetadata: ActionGetResponse = {
    icon: "https://www.rockstaracademy.com/lib/images/news/basketball.jpeg",
    title: "Vote for your favorite basketball player",
    description:
      "Vote for your favorite basketball player and show your support!",
    label: "Vote",
    links: {
      actions: [
        {
          type: "transaction",
          label: "Vote for LeBron James",
          href: "/api/vote?candidate=LeBronJames",
        },
        {
          type: "transaction",
          label: "Vote for Stephen Curry",
          href: "/api/vote?candidate=StephenCurry",
        },
      ],
    },
  };
  return Response.json(actionMetadata, { headers: ACTIONS_CORS_HEADERS });
}

export async function POST(request: Request) {
  const url = new URL(request.url);
  let candidate = url.searchParams.get("candidate");

  if (candidate === "LeBronJames") {
    candidate = "LeBron James";
  } else if (candidate === "StephenCurry") {
    candidate = "Stephen Curry";
  } else {
    return new Response("Invalid candidate", {
      status: 400,
      headers: ACTIONS_CORS_HEADERS,
    });
  }

  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const program: Program<Voting> = new Program(IDL, { connection });

  const body: ActionPostRequest = await request.json();
  let voter;

  try {
    voter = new PublicKey(body.account);
  } catch (error) {
    return new Response("Invalid account", {
      status: 400,
      headers: ACTIONS_CORS_HEADERS,
    });
  }

  const instruction = await program.methods
    .vote(candidate, new BN(1))
    .accounts({ signer: voter })
    .instruction();

  const blockhash = await connection.getLatestBlockhash();

  const transaction = new Transaction({
    feePayer: voter,
    blockhash: blockhash.blockhash,
    lastValidBlockHeight: blockhash.lastValidBlockHeight,
  }).add(instruction);

  const response = await createPostResponse({
    fields: { transaction: transaction, type: "transaction" },
  });

  return Response.json(response, { headers: ACTIONS_CORS_HEADERS });
}
