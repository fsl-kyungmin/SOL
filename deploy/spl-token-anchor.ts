// Anchor 버전
// async function mintToken() {
//   const mint = Keypair.generate();

//   const space = MintLayout.span;
//   const lamports = await connection.getMinimumBalanceForRentExemption(space);
//   const createMintIx = SystemProgram.createAccount({
//     fromPubkey: user.publicKey,
//     newAccountPubkey: mint.publicKey,
//     lamports,
//     space,
//     programId: SPL_PROGRAM_ID,
//   });
//   const transaction = new Transaction();
//   const tx = transaction.add(createMintIx);
//   await sendAndConfirmTransaction(connection, tx, [user, mint]);

//   await tokenProgram.methods
//     .initialize(6)
//     .accounts({
//       mint: mint.publicKey,
//       authority: mint.publicKey,
//       rent: SYSVAR_RENT_PUBKEY,
//       system_program: SystemProgram.programId,
//       token_program: SPL_PROGRAM_ID,
//     })
//     .signAndSend(user)
//     .rpc();

//   console.log("My Token CA:", mint.publicKey.toBase58());
//   return mint.publicKey.toBase58();
// }
