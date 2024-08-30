import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { AnchorSwapNft } from "../target/types/anchor_swap_nft";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createNft,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { createMint, getAssociatedTokenAddress } from "@solana/spl-token";
import {
  generateSigner,
  percentAmount,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
describe("anchor-swap-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorSwapNft as Program<AnchorSwapNft>;

  const [maker, user1, user2] = [
    web3.Keypair.generate(),
    web3.Keypair.generate(),
    web3.Keypair.generate(),
  ];

  console.table({
    maker: maker.publicKey.toBase58(),
    user1: user1.publicKey.toBase58(),
    user2: user2.publicKey.toBase58(),
  });

  // const umi = createUmi("http://127.0.0.1:8899").use(mplTokenMetadata());
  // .use(walletAdapterIdentity(provider.wallet, true));

  // const umiSigner = generateSigner(umi);

  // umi.use(signerIdentity(umiSigner));

  const [mintX, mintY] = [web3.Keypair.generate(), web3.Keypair.generate()];

  console.table({
    mintX: mintX.publicKey.toBase58(),
    mintY: mintY.publicKey.toBase58(),
  });

  before("Setup", async () => {
    {
      await provider.connection.confirmTransaction({
        signature: await provider.connection.requestAirdrop(
          maker.publicKey,
          10 * anchor.web3.LAMPORTS_PER_SOL
        ),
        ...(await provider.connection.getLatestBlockhash()),
      });
      await provider.connection.confirmTransaction({
        signature: await provider.connection.requestAirdrop(
          new web3.PublicKey(maker.publicKey),
          10 * anchor.web3.LAMPORTS_PER_SOL
        ),
        ...(await provider.connection.getLatestBlockhash()),
      });
      await provider.connection.confirmTransaction({
        signature: await provider.connection.requestAirdrop(
          user2.publicKey,
          10 * anchor.web3.LAMPORTS_PER_SOL
        ),
        ...(await provider.connection.getLatestBlockhash()),
      });
    }
    {
      // const associatedTokenAccount = await getAssociatedTokenAddress(
      //   mintX.publicKey,
      //   provider.publicKey
      // );
      // let [metadataAccount] = findMetadataPda(umi, {
      //   mint: publicKey(mintX.publicKey),
      // });
      // let [masterEditionAccount] = findMasterEditionPda(umi, {
      //   mint: publicKey(mintX.publicKey),
      // });
      // const mint = generateSigner(umi);
      // const metadata = {
      //   name: "ONE NFT",
      //   symbol: "ONFT",
      //   uri: "https://example.com/my-nft.json",
      // };
      // await new Promise((resolve) => {
      //   setTimeout(resolve, 10000);
      // });
      // await createNft(umi, {
      //   mint: mint,
      //   name: metadata.name,
      //   uri: metadata.uri,
      //   sellerFeeBasisPoints: percentAmount(5.5),
      // }).sendAndConfirm(umi);
    }
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(100)
      .accounts({
        signer: provider.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
