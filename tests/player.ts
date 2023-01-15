import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import { assert } from "chai";
import { Game } from "../target/types/game";


describe("player", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Game as Program<Game>;


  it("Is initialized!", async () => {


    const publicKey = anchor.AnchorProvider.local().wallet.publicKey;
    const toWallet: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    const [PlayerPDA] = await anchor.web3.PublicKey.findProgramAddressSync([
      utf8.encode('user-stats'),
      toWallet.publicKey.toBuffer()
    ],
      program.programId
    );
    console.log("PlayerPDA", PlayerPDA);
    const tx = await program.methods.createUserStats("player", 10, true).accounts({ user_stats: PlayerPDA, user: toWallet.publicKey, systemProgram: anchor.web3.SystemProgram.programId, }).rpc();
    // Add your test here.ti
    // await program.methods.create_user_stats({
    //   accounts:{
    //     myAccount: myAccount.publicKey,
    //     user: provider.wallet.publicKey,
    //     SystemProgram: SystemProgram.programId,
    //   },
    //   name:"player1",
    //   amount:0,
    //   action:true,
    //   signers: [myAccount],
    // })
    // const account = await program.account.myAccount.fetch(myAccount.publicKey);
    // assert.ok(account.data.eq(new anchor.BN(0)));
    // console.log("Your transaction signature", tx);
  });



});
