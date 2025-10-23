import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Bullorbear } from '../target/types/bullorbear'

describe('bullorbear', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Bullorbear as Program<Bullorbear>

  const bullorbearKeypair = Keypair.generate()
})
