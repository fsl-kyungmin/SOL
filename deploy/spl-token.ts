import "dotenv/config";
import {
  getKeypairFromEnvironment,
  getExplorerLink,
} from "@solana-developers/helpers";
import {
  Connection,
  clusterApiUrl,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { createInitializeMintInstruction, MINT_SIZE } from "@solana/spl-token";
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";

const SPL_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const user = getKeypairFromEnvironment("SECRET_KEY");
const connection = new Connection(clusterApiUrl("devnet"));

async function mintToken() {
  const mint = Keypair.generate();

  const space = MINT_SIZE;
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  const createMintIx = SystemProgram.createAccount({
    fromPubkey: user.publicKey,
    newAccountPubkey: mint.publicKey,
    lamports,
    space,
    programId: SPL_PROGRAM_ID,
  });

  const initializeMintIx = createInitializeMintInstruction(
    mint.publicKey,
    6,
    user.publicKey,
    user.publicKey,
    SPL_PROGRAM_ID
  );

  const transaction = new Transaction().add(createMintIx).add(initializeMintIx);
  await sendAndConfirmTransaction(connection, transaction, [user, mint]);
  console.log("My Token Mint Address:", mint.publicKey.toBase58());
  return mint.publicKey.toBase58();
}

async function registerMetaData(tokenMintAccount: PublicKey) {
  const metadataPDAAndBump = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      tokenMintAccount.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const metadataPDA = metadataPDAAndBump[0];

  const metadataData = {
    name: "T-Game Club",
    symbol: "T-GC",
    uri: "https://acbs-test.s3.ap-northeast-2.amazonaws.com/metadata.json",
    sellerFeeBasisPoints: 0,
    creators: null,
    collection: null,
    uses: null,
  };

  const transaction = new Transaction();
  const createMetadataAccountInstruction =
    createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA,
        mint: tokenMintAccount,
        mintAuthority: user.publicKey,
        payer: user.publicKey,
        updateAuthority: user.publicKey,
      },
      {
        createMetadataAccountArgsV3: {
          collectionDetails: null,
          data: metadataData,
          isMutable: true,
        },
      }
    );

  transaction.add(createMetadataAccountInstruction);

  await sendAndConfirmTransaction(connection, transaction, [user]);

  const tokenMintLink = getExplorerLink(
    "address",
    tokenMintAccount.toString(),
    "devnet"
  );

  console.log(`Finish: ${tokenMintLink}!`);
}

(async () => {
  const mintAddress = await mintToken();
  await registerMetaData(new PublicKey(mintAddress));
})();
