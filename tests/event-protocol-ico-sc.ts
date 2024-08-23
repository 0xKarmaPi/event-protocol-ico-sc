import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EventProtocolIcoSc } from "../target/types/event_protocol_ico_sc";
import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js";
import { getAssociatedTokenAccount, initWalletWithSols } from "./helper";
import * as fs from "fs";
import { Keypair } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";

describe("event-protocol-ico-sc", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace
    .EventProtocolIcoSc as Program<EventProtocolIcoSc>;

  const metadata = {
    name: "Julian Non Fungible Token",
    symbol: "JNFT",
    description: "Julian Non Fungible  desctiption ....",
    image:
      "https://nexgard.com.au/sites/default/files/2024-02/AdobeStock274064877_360x316.jpeg",
    sellerFeeBasisPoints: 1000, // Represents 10.00%
  };

  it("Hello function!", async () => {
    // Add your test here.
    const tx = await program.methods.hello().rpc();
    console.log("Your transaction signature", tx);
  });

  it("init a wrapper without Metaplex", async () => {
    // Create a new mint for the NFT
    const tokenMint = await createMint(
      connection,
      provider.wallet,
      provider.wallet.publicKey,
      null,
      0 // Set decimals to 0 for an NFT
    );

    // Create a token account to hold the NFT
    const tokenAccount = await createAccount(
      connection,
      provider.wallet.payer,
      tokenMint,
      provider.wallet.publicKey
    );

    // Mint 1 NFT to the token account
    await mintTo(
      connection,
      provider.wallet.payer,
      tokenMint,
      tokenAccount,
      provider.wallet.publicKey,
      1
    );

    // Generate a new keypair for the metadata account
    const metadataAccount = Keypair.generate();
    const collectionKey = Keypair.generate().publicKey;

    // Metadata for the NFT
    const data: DataV2 = {
      name: metadata.name,
      symbol: metadata.symbol,
      uri: metadata.uri,
      sellerFeeBasisPoints: metadata.sellerFeeBasisPoints,
      creators: null, // Optional: Add creators if needed
      collection: { key: collectionKey, verified: false }, // Add the collection
      uses: null,
    };

    // Create metadata account instruction
    const createMetadataInstruction = createCreateMetadataAccountV2Instruction(
      {
        metadata: metadataAccount.publicKey,
        mint: tokenMint,
        mintAuthority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
        updateAuthority: provider.wallet.publicKey,
      },
      {
        createMetadataAccountArgsV2: {
          data,
          isMutable: true,
        },
      }
    );

    // Send transaction to create the metadata account
    const transaction = new anchor.web3.Transaction().add(
      createMetadataInstruction
    );
    await provider.sendAndConfirm(transaction, [metadataAccount]);

    console.log("NFT Mint Address:", tokenMint.toBase58());
    console.log("Metadata Account:", metadataAccount.publicKey.toBase58());
    console.log("Collection Key:", collectionKey.toBase58());

    // Now, you can call your program's initAWrapper method
    const tx = await program.methods
      .initAWrapper(tokenMint, collectionKey)
      .accounts({
        signer: provider.wallet.publicKey,
        wrapper: Keypair.generate().publicKey, // Replace with actual wrapper PDA if needed
        master: Keypair.generate().publicKey, // Replace with actual master PDA if needed
        senderNftAccount: tokenAccount,
        mintMetadataAccount: metadataAccount.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature:", tx);
  });

  // it("init a wrapper", async () => {
  //   const metaplex = Metaplex.make(connection)
  //     .use(
  //       keypairIdentity(
  //         Keypair.fromSecretKey(
  //           Uint8Array.from(
  //             JSON.parse(
  //               fs.readFileSync(
  //                 "/Users/lainhathoang/.config/solana/id.json",
  //                 "utf8"
  //               )
  //             )
  //           )
  //         )
  //       )
  //     )
  //     .use(
  //       bundlrStorage({
  //         address: "https://devnet.bundlr.network",
  //         providerUrl: "https://api.devnet.solana.com",
  //         timeout: 60000,
  //       })
  //     );
  //   console.log("Uploading metadata...");
  //   const { uri } = await metaplex.nfts().uploadMetadata(metadata);
  //   console.log("Metadata uploaded:", uri);
  //   console.log("Creating NFT using Metaplex...");
  //   const tokenMint = Keypair.generate();
  //   const collection = Keypair.generate();
  //   const { nft, response } = await metaplex.nfts().create({
  //     uri,
  //     name: metadata.name,
  //     symbol: metadata.symbol,
  //     useNewMint: tokenMint,
  //     // `sellerFeeBasisPoints` is the royalty that you can define on nft
  //     sellerFeeBasisPoints: 1000, // Represents 10.00%.
  //     isMutable: true,
  //     collection: collection.publicKey,
  //   });
  //   console.log(nft);
  //   const tx = await program.methods.hello().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("claim function", async () => {
  //   const signer = await initWalletWithSols(connection, 10);

  //   const wrapperId = new anchor.BN(1);
  //   const wrapperNftAddress = anchor.web3.Keypair.generate();

  //   const [wrapperPda] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("wrapper"), wrapperNftAddress.publicKey.toBuffer()],
  //     program.programId
  //   );

  //   const txBuyAWrapper = await program.methods
  //     .initAWrapper(wrapperNftAddress.publicKey)
  //     .accounts({
  //       signer: signer.publicKey,
  //       wrapper: wrapperPda,
  //     })
  //     .signers([signer])
  //     .rpc();
  //   console.log("=> txBuyAWrapper: ", txBuyAWrapper);

  //   const txClaim = await program.methods
  //     .claim(wrapperNftAddress.publicKey)
  //     .accounts({
  //       signer: signer.publicKey,
  //       wrapper: wrapperPda,
  //     })
  //     .signers([signer])
  //     .rpc();

  //   console.log("=> txClaim: ", txClaim);
  // });
});
