import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LotteryContract } from "../target/types/lottery_contract";
import {Connection, LAMPORTS_PER_SOL} from '@solana/web3.js'

import { SolanaConfigService} from '@coin98/solana-support-library/config'
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { SolanaService } from "@coin98/solana-support-library";


describe("lottery-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LotteryContract as Program<LotteryContract>;
  const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

  let root : anchor.web3.Keypair;
  let player0 : anchor.web3.Keypair;
  let player1: anchor.web3.Keypair;
  let player2: anchor.web3.Keypair;

  before(async () => {
    root = await SolanaConfigService.getDefaultAccount();
    console.log('Payer', root.publicKey.toBase58());
    player0 = anchor.web3.Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(player0.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature);

    player1 = anchor.web3.Keypair.generate();
    const airdropSignature1 = await connection.requestAirdrop(player1.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature1);

    player2 = anchor.web3.Keypair.generate();
    const airdropSignature2 = await connection.requestAirdrop(player2.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature2);
})

  it("Lottery for solana", async () => {

    const [lotteryMasterPda, lotteryMasterBump] = findProgramAddressSync([Buffer.from('INIT_LOTTERY'), root.publicKey.toBuffer()], program.programId);

    console.log('program', program.programId.toString());

    const tx = await program.methods.initLotteryMaster().accounts({
      root: root.publicKey,
      lotteryMaster: lotteryMasterPda,
    }).signers([root]).rpc();

    console.log('Init lottery master transaction', tx);

    const lotteryMasterAccount = await program.account.lotteryMaster.fetch(lotteryMasterPda);
    console.log('Lottery account', lotteryMasterAccount);

    const [lotteryPda, lotteryBump] = findProgramAddressSync([Buffer.from('INIT_LOTTERY'), Buffer.from([lotteryMasterAccount.lotteryCount])], program.programId);
    
    const [lotteryWalletPda, lotteryWalletBump] = findProgramAddressSync([Buffer.from('LOTTERY_WALLET'), Buffer.from([lotteryMasterAccount.lotteryCount])], program.programId);
    

    console.log(`Lottery PDA ${lotteryPda}, ${lotteryBump}` );

    const txInitLottery = await program.methods.initLottery().accounts({
      root: root.publicKey,
      lotteryMaster: lotteryMasterPda,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([root]).rpc();

    console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

    await program.methods.addMoneyToLottery(0).accounts({
      player: player0.publicKey,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([player0]).rpc();

    console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

    await program.methods.addMoneyToLottery(0).accounts({
      player: player1.publicKey,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([player1]).rpc();

    console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

    await program.methods.addMoneyToLottery(0).accounts({
      player: player2.publicKey,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([player2]).rpc();

    console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

    const txPickWinner = await program.methods.pickWinner(0).accounts({
      root: root.publicKey,
      lotteryAccount: lotteryPda,
    }).signers([root]).rpc();

    console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

    console.log('Transaction', txPickWinner);

    try{    
        const txClaim0 = await program.methods.claim(0, lotteryWalletBump).accounts({
        player: player0.publicKey,
        lotteryAccount: lotteryPda,
        lotterySigner: lotteryWalletPda,
      }).signers([player0]).rpc();
    }
    catch(error) {
      console.log(error.logs);
    }

    await new Promise(f => setTimeout(f, 1000));
    let balancePlayer0 = await connection.getBalance(player0.publicKey);
    console.log('Balance of player 0', balancePlayer0/LAMPORTS_PER_SOL);

    try{
      const txClaim1 = await program.methods.claim(0, lotteryWalletBump).accounts({
      player: player1.publicKey,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([player1]).rpc();
    }
    catch(error) {
      console.log(error.logs);
    }

    await new Promise(f => setTimeout(f, 1000));
    let balancePlayer1 = await connection.getBalance(player1.publicKey);
    console.log('Balance of player 1', balancePlayer1 / LAMPORTS_PER_SOL);

    try{
      const txClaim2 = await program.methods.claim(0, lotteryWalletBump).accounts({
      player: player2.publicKey,
      lotteryAccount: lotteryPda,
      lotterySigner: lotteryWalletPda,
    }).signers([player2]).rpc();
    }
    catch(error) {
      console.log(error.logs);
    }
    await new Promise(f => setTimeout(f, 1000));
    let balancePlayer2 = await connection.getBalance(player2.publicKey);
    console.log('Balance of player 2', balancePlayer2 / LAMPORTS_PER_SOL);
  });

});
