import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolPredictionMarket } from "../target/types/sol_prediction_market";
import dotenv from "dotenv"; 
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import {ObjectId} from "bson";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";


dotenv.config({ path: "./tests/.env" });

describe("sol-prediction-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solPredictionMarket as Program<SolPredictionMarket>;

  const connection = new Connection("https://api.devnet.solana.com");

  const authorityKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TEST_KEYPAIR!));
  const userKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TEST2_KEYPAIR!));
  
  const decimals = new BN(10).pow(new BN(1));
  
  const usdcMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  const usdcDecimals = new BN(10).pow(new BN(6));
  
  const MARKET_ID = new ObjectId("690bbe4892b93c91540ad467");

  const marketAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("market"), MARKET_ID.id],
      program.programId
    )[0];

    const outcomeAMint = PublicKey.findProgramAddressSync(
      [Buffer.from("outcome_a"), marketAccount.toBuffer()],
      program.programId
    )[0];

    const outcomeBMint = PublicKey.findProgramAddressSync(
      [Buffer.from("outcome_b"), marketAccount.toBuffer()],
      program.programId
    )[0];

    const baseTokenVault = getAssociatedTokenAddressSync(
      usdcMint,
      marketAccount,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

  it("Initialize vault", async () => {
    
    let marketAccountInfo = await connection.getAccountInfo(marketAccount);
    if (!marketAccountInfo) {
      const tx = await program.methods
        .initializeMarket(Array.from(MARKET_ID.id))
        .accounts({
          authority: authorityKeypair.publicKey,
          marketAccount: marketAccount,
          outcomeAMint: outcomeAMint,
          outcomeBMint: outcomeBMint,
          baseTokenMint: usdcMint,
          baseTokenVault: baseTokenVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([authorityKeypair])
        .rpc();
        console.log("Your transaction signature", tx);
    }
    const marketAccountData = await program.account.market.fetch(marketAccount);
    
    expect(marketAccountData.authority.equals(authorityKeypair.publicKey)).to.be.true;
    expect(marketAccountData.marketId).to.deep.equal(Array.from(MARKET_ID.id));
    expect(marketAccountData.outcomeAMint.equals(outcomeAMint)).to.be.true;
    expect(marketAccountData.outcomeBMint.equals(outcomeBMint)).to.be.true;
    expect(marketAccountData.baseTokenMint.equals(usdcMint)).to.be.true;
    expect(marketAccountData.baseTokenVault.equals(baseTokenVault)).to.be.true;
    expect(marketAccountData.isSettled).to.be.false;
    expect(marketAccountData.winningOutcome).to.null;
  });
});
