import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import {
  MintLayout,
  AccountLayout,
  createInitializeAccountInstruction,
} from "@solana/spl-token";
const { SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;

describe("spl-token-devnet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  //@ts-ignore
  const program = anchor.workspace.SplTokenDevnet as anchor.Program<any>;

  const tokenProgramId = new anchor.web3.PublicKey(
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  );

  const mint = anchor.web3.Keypair.generate();
  const tokenAccount = anchor.web3.Keypair.generate();

  it("Create Mint Account", async () => {
    const space = MintLayout.span;
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(space);
    const createMintIx = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint.publicKey,
      lamports,
      space,
      programId: tokenProgramId,
    });
    const tx = new anchor.web3.Transaction().add(createMintIx);
    await provider.sendAndConfirm(tx, [mint]);
  });

  it("Initialize Mint", async () => {
    await program.methods
      .initialize(6)
      .accounts({
        mint: mint.publicKey,
        authority: provider.wallet.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        system_program: SystemProgram.programId,
        token_program: tokenProgramId,
      })
      .rpc();

    const mintInfo = await provider.connection.getAccountInfo(mint.publicKey);
    if (!mintInfo) throw new Error("Mint account not found");
    const decodedMint = MintLayout.decode(mintInfo.data);
    expect(decodedMint.decimals).to.equal(6);
    console.log("My Token CA:", mint.publicKey.toBase58());
  });

  it("Create Token Account & Mint Tokens", async () => {
    const tokenAccountSpace = AccountLayout.span;
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(
        tokenAccountSpace
      );
    const createTokenAccountIx = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: tokenAccount.publicKey,
      lamports,
      space: tokenAccountSpace,
      programId: tokenProgramId,
    });

    const initializeTokenAccountIx = createInitializeAccountInstruction(
      tokenAccount.publicKey,
      mint.publicKey,
      provider.wallet.publicKey,
      tokenProgramId
    );
    const tx = new anchor.web3.Transaction().add(
      createTokenAccountIx,
      initializeTokenAccountIx
    );
    await provider.sendAndConfirm(tx, [tokenAccount]);

    await program.methods
      .mintTokens(new anchor.BN(10000000000000))
      .accounts({
        mint: mint.publicKey,
        tokenAccount: tokenAccount.publicKey,
        authority: provider.wallet.publicKey,
        token_program: tokenProgramId,
      })
      .rpc();

    const tokenAccInfo = await provider.connection.getAccountInfo(
      tokenAccount.publicKey
    );
    if (!tokenAccInfo) throw new Error("Token account not found");
    const decodedTokenAcc = AccountLayout.decode(tokenAccInfo.data);
    const amount = new anchor.BN(decodedTokenAcc.amount, "le").toNumber();

    expect(amount).to.equal(10000000000000);
  });
});
