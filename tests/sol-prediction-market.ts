import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolPredictionMarket } from "../target/types/sol_prediction_market";
import dotenv from "dotenv"; 
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import {ObjectId} from "bson";
import { ASSOCIATED_TOKEN_PROGRAM_ID, AuthorityType, getAccount, getAssociatedTokenAddressSync, setAuthority, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert, expect } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";



dotenv.config({ path: "./tests/.env" });

describe("sol-prediction-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solPredictionMarket as Program<SolPredictionMarket>;

  const authorityKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TEST_KEYPAIR!));
  const userKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TEST2_KEYPAIR!));
  const RPC_URL = process.env.RPC_URL;

   const connection = new Connection(RPC_URL);
  
  const decimals = new BN(10).pow(new BN(6));
  
  const usdcMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  const usdcDecimals = new BN(10).pow(new BN(6));
  
  const MARKET_ID = new ObjectId("690bbe4892b93c91540ad570");

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

    const userUsdcAta = getAssociatedTokenAddressSync(
      usdcMint,
      userKeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const marketOutcomeAAta = getAssociatedTokenAddressSync(
      outcomeAMint,
      marketAccount,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const marketOutcomeBAta = getAssociatedTokenAddressSync(
      outcomeBMint,
      marketAccount,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userOutcomeAAta = getAssociatedTokenAddressSync(
      outcomeAMint,
      userKeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userOutcomeBAta = getAssociatedTokenAddressSync(
      outcomeBMint,
      userKeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log("Market Account = " + marketAccount.toString());
    console.log("outcome A mint = " + outcomeAMint.toString());
    console.log("outcome B mint = " + outcomeBMint.toString());
    console.log("Base token vault = " + baseTokenVault.toString());
    console.log("User Usdc Ata = " + userUsdcAta.toString());
    console.log("Market Outcome A Ata = " + marketOutcomeAAta.toString());
    console.log("Market Outcome B Ata = " + marketOutcomeBAta.toString());
    console.log("User Outcome A Ata = " + userOutcomeAAta.toString());
    console.log("User Outcome B Ata = " + userOutcomeBAta.toString());

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

  it("Split", async () => {
    const tx = await program.methods
      .split(Array.from(MARKET_ID.id), new BN(5).mul(decimals))
      .accounts({
        authority: authorityKeypair.publicKey,
        user: userKeypair.publicKey,
        marketAccount: marketAccount,
        outcomeAMint: outcomeAMint,
        outcomeBMint: outcomeBMint,
        baseTokenMint: usdcMint,
        baseTokenVault: baseTokenVault,
        userOutcomeAAta: userOutcomeAAta,
        userOutcomeBAta: userOutcomeBAta,
        userBaseTokenAta: userUsdcAta,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([authorityKeypair, userKeypair])
      .rpc();

    console.log ("TX: " + tx);

    const userOutcomeAAtaInfo = await getAccount(connection, userOutcomeAAta);
    const userOutcomeBAtaInfo = await getAccount(connection, userOutcomeBAta);

    expect(userOutcomeAAtaInfo.amount.toString()).to.equal(new BN (5).mul(decimals).toString());
    expect(userOutcomeBAtaInfo.amount.toString()).to.equal(new BN (5).mul(decimals).toString());

  });

  it("Merge", async () => {
    const tx = await program.methods
      .merge(Array.from(MARKET_ID.id), new BN(2).mul(decimals))
      .accounts({
        authority: authorityKeypair.publicKey,
        user: userKeypair.publicKey,
        marketAccount: marketAccount,
        outcomeAMint: outcomeAMint,
        outcomeBMint: outcomeBMint,
        baseTokenMint: usdcMint,
        baseTokenVault: baseTokenVault,
        userOutcomeAAta: userOutcomeAAta,
        userOutcomeBAta: userOutcomeBAta,
        userBaseTokenAta: userUsdcAta,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([authorityKeypair, userKeypair])
      .rpc();

    console.log ("TX: " + tx);

    const userOutcomeAAtaInfo = await getAccount(connection, userOutcomeAAta);
    const userOutcomeBAtaInfo = await getAccount(connection, userOutcomeBAta);

    expect(userOutcomeAAtaInfo.amount.toString()).to.equal(new BN (3).mul(decimals).toString());
    expect(userOutcomeBAtaInfo.amount.toString()).to.equal(new BN (3).mul(decimals).toString());

  });

  it("Set Winning Side", async () => {
    const tx = await program.methods
      .setWinningSide(Array.from(MARKET_ID.id), { outcomeB: {} })
      .accounts({
        authority: authorityKeypair.publicKey,
        marketAccount: marketAccount,
        outcomeAMint: outcomeAMint,
        outcomeBMint: outcomeBMint,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([authorityKeypair])
      .rpc()

      console.log ("TX: " + tx);  

    const marketInfo = await program.account.market.fetch(marketAccount);
  
    expect(marketInfo.isSettled).to.be.true;
    expect(marketInfo.winningOutcome).to.deep.equal({ outcomeB: {} })
  });

  it("Claim Rewards", async () => {
    const tx = await program.methods
      .claimRewards(Array.from(MARKET_ID.id))
      .accounts({
        authority: authorityKeypair.publicKey,
        user: userKeypair.publicKey,
        marketAccount: marketAccount,
        outcomeAMint: outcomeAMint,
        outcomeBMint: outcomeBMint,
        baseTokenMint: usdcMint,
        baseTokenVault: baseTokenVault,
        userOutcomeAAta: userOutcomeAAta,
        userOutcomeBAta: userOutcomeBAta,
        userBaseTokenAta: userUsdcAta,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([authorityKeypair, userKeypair])
      .rpc();

    console.log ("TX: " + tx); 

    const userOutcomeAAtaInfo = await getAccount(connection, userOutcomeAAta);
    const userOutcomeBAtaInfo = await getAccount(connection, userOutcomeBAta);

    expect(userOutcomeAAtaInfo.amount.toString()).to.equal(new BN (0).mul(decimals).toString());
    expect(userOutcomeBAtaInfo.amount.toString()).to.equal(new BN (0).mul(decimals).toString());

  });

});
