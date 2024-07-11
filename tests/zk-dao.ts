import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ZkDao } from "../target/types/zk_dao";

describe("zk_dao", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ZkDao as Program<ZkDao>;

  it("Creates a proposal", async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal("Test Proposal", {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    console.log("Proposal:", proposalAccount);
  });

  it("Votes on a proposal", async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal("Test Proposal for Voting", {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    const voter = anchor.web3.Keypair.generate();
    await program.rpc.vote(true, {
      accounts: {
        proposal: proposal.publicKey,
        voter: voter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [voter],
    });

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    console.log("Updated Proposal:", proposalAccount);
  });

  it("Displays results", async () => {
    const proposal = anchor.web3.Keypair.generate();
    await program.rpc.createProposal("Test Proposal for Results", {
      accounts: {
        proposal: proposal.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [proposal],
    });

    await program.rpc.getResults({
      accounts: {
        proposal: proposal.publicKey,
      },
    });
  });
});
