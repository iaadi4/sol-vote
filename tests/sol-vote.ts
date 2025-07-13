import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolVote } from "../target/types/sol_vote";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("sol-vote", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.solVote as Program<SolVote>;

  let user: Keypair;
  let pollCounterPda: PublicKey;
  let pollPda: PublicKey;

  before(async () => {
    user = Keypair.generate();
    const sig = await provider.connection.requestAirdrop(user.publicKey, 1e9);
    await provider.connection.confirmTransaction(sig);

    [pollCounterPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("poll_counter")],
      program.programId
    );

    let counterAccount;
    try {
      counterAccount = await program.account.pollCounter.fetch(pollCounterPda);
    } catch {
      counterAccount = null;
    }
    const pollId = counterAccount ? (counterAccount.count as BN) : new BN(0);

    [pollPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),
        user.publicKey.toBuffer(),
        pollId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .initialize(
        "Test Poll",
        "This is a test poll",
        ["Option 1", "Option 2", "Option 3"],
        new BN(3600)
      )
      .accounts({
        user:           user.publicKey,
        pollCounter:    pollCounterPda,
        pollAccount:    pollPda,
        systemProgram:  SystemProgram.programId,
      })
      .signers([user])
      .rpc();
  });

  it("initializes poll correctly", async () => {
    const pollData = await program.account.pollData.fetch(pollPda);
    expect(pollData.pollId.toNumber()).to.equal(0);
    expect(pollData.title).to.equal("Test Poll");
    expect(pollData.options.length).to.equal(3);
    expect(pollData.votes.every(v => v.toNumber() === 0)).to.be.true;
  });

  it("casts a vote and updates count", async () => {
    const [userVotePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("user_vote"),
        user.publicKey.toBuffer(),
        pollPda.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .castVote(1)
      .accounts({
        userVote:       userVotePda,
        user:           user.publicKey,
        pollAccount:    pollPda,
        systemProgram:  SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const updated = await program.account.pollData.fetch(pollPda);
    console.log("Updated Poll Data:", updated);
    expect(updated.votes[1].toNumber()).to.equal(1);
  });
});
