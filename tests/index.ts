import {
  Program,
  Idl,
  Provider,
  web3,
  AnchorProvider,
  Wallet,
  setProvider,
  BN,
} from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import * as fs from "fs";
import { EventProtocolIcoSc } from "../target/types/event_protocol_ico_sc";
import ico_idl from "../target/idl/event_protocol_ico_sc.json";
import { IDL } from "@coral-xyz/anchor/dist/cjs/native/system";
import { PublicKey } from "@solana/web3.js";

export function loadWalletKey(keyPairFile: string): web3.Keypair {
  const kp = web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keyPairFile, "utf-8")))
  );
  return kp;
}

(async function main() {
  const connection = new web3.Connection("https://api.devnet.solana.com");
  const keypair = loadWalletKey("/Users/lainhathoang/Documents/Work/twendee/karmapi/event-nfts/candy-machine-v3/another-minter.json");
  const wallet = new Wallet(keypair);
  const provider = new AnchorProvider(connection, wallet, {});
  setProvider(provider);

  const programId = new web3.PublicKey(
    "H7PgZvp7cA8EBNkyh6TWc1rwJ1JbsZRE65fgRtmJFB6S"
  );

  const collectionAddress = new web3.PublicKey(
    "8NFNd1YKhifysCJRnp4tiHcv9C9oq7ctwbUyMwt41Rrx"
  );

  // const idlFetched = await Program.fetchIdl<EventProtocolIcoSc>(
  //   programId,
  //   provider
  // );

  const program = new Program(ico_idl as EventProtocolIcoSc, provider);

  //   const txHello = await program.methods.hello().accounts({}).rpc();
  //   console.log("txHello: ", txHello);

  const [masterPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("master")],
    programId
  );

  // const txInitMaster = await program.methods
  //   .initMaster(collectionAddress)
  //   .accountsStrict({
  //     master: masterPda,
  //     signer: wallet.publicKey,
  //     systemProgram: web3.SystemProgram.programId,
  //   })
  //   .signers([wallet.payer])
  //   .rpc();
  // console.log("initMaster: ", txInitMaster);

  // ================
  const masterPdaFetched = await program.account.master.fetch(masterPda);
  console.log("masterPdaFetched: ", masterPdaFetched.owner.toBase58());

  const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const mintOfNft = new web3.PublicKey(
    "8vMeoHz3zdhx9Qn3PbgZTchQmJnRteG6hqGkJe1bFMAa"
  );

  const senderNftAccount = await spl.getAssociatedTokenAddress(
    mintOfNft,
    wallet.publicKey
  );

  const [metadataPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintOfNft.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

  const [wrapperPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("wrapper"), mintOfNft.toBuffer()],
    programId
  );

  console.log("senderNftAccount: ", senderNftAccount.toBase58());
  console.log("metadataPda: ", metadataPda.toBase58());
  console.log("wrapperPda: ", wrapperPda.toBase58());
  // ================

  const initWrapperTx = await program.methods
    .initAWrapper(mintOfNft, collectionAddress)
    .accountsStrict({
      master: masterPda,
      wrapper: wrapperPda,
      mintOfNft,
      senderNftAccount,
      mintMetadataAccount: metadataPda,
      signer: wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([wallet.payer])
    .rpc();

  console.log("initWrapperTx: ", initWrapperTx);

  const wrapperAccount = await program.account.wrapper.fetch(wrapperPda);
  // console.log("wrapperAccount: ", wrapperAccount);
  console.log(wrapperAccount.amountOfTokens.toNumber());
  console.log(wrapperAccount.amountOfTokensClaimed.toNumber());
  console.log(wrapperAccount.nftAddress.toBase58());
  console.log(wrapperAccount.initTime.toNumber());
  console.log(wrapperAccount.startTime.toNumber());
})();
