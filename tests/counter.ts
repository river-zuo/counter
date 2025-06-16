import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Counter as Program<Counter>;

  const user = anchor.AnchorProvider.env().wallet;
  const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Initialize counter", async () => {
    await program.methods
      .initialize()
      .accounts({
        counterAccount: counterPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    let account = await program.account.counter.fetch(counterPda);
    console.log("Initial count:", account.count.toString()); // expect 0
  });

  it("Increment counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counterAccount: counterPda,
        user: user.publicKey,
      })
      .rpc();

    let account = await program.account.counter.fetch(counterPda);
    console.log("Updated count:", account.count.toString()); // expect 1
  });
});
