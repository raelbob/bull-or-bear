// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, IdlAccounts, IdlEvents, Program, Wallet } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import BullorbearIDL from '../target/idl/bullorbear.json'
import type { Bullorbear } from '../target/types/bullorbear'

// Re-export the generated IDL and type
export { Bullorbear, BullorbearIDL }

// The programId is imported from the program IDL.
export const BULLORBEAR_PROGRAM_ID = new PublicKey(BullorbearIDL.address)

// This is a helper function to get the Bullorbear Anchor program.
export function getBullorbearProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...BullorbearIDL, address: address ? address.toBase58() : BullorbearIDL.address } as Bullorbear, provider)
}

// type BullorbearProgram = ReturnType<typeof getBullorbearProgram>;
// type Round = Awaited<ReturnType<BullorbearProgram['account']['round']['fetch']>>;
// type AccountType<
//   P extends { account: Record<string, any> },
//   K extends keyof P['account']
// > = Awaited<ReturnType<P['account'][K]['fetch']>>;

// export type Round = AccountType<ReturnType<typeof getBullorbearProgram>, 'round'>;

export type Round = IdlAccounts<Bullorbear>['round'];
export type Config = IdlAccounts<Bullorbear>['config'];
export type UserBet = IdlAccounts<Bullorbear>['bet'];

export type RoundInitializedEvent = IdlEvents<Bullorbear>['roundInitialized'];
export type RoundStartedEvent = IdlEvents<Bullorbear>['roundStarted'];
export type RoundClosedEvent = IdlEvents<Bullorbear>['roundClosed'];
export type GenesisInitializedEvent = IdlEvents<Bullorbear>['genesisInitialized'];
export type BetEvent = IdlEvents<Bullorbear>['betEvent'];
export type ClaimEvent = IdlEvents<Bullorbear>['claim'];

export function getRoundAddress(programId: PublicKey, currentEpoch: number) {
  const epochBytes = Buffer.from(new Uint8Array(new BigUint64Array([BigInt(currentEpoch)]).buffer));
  const [address] = PublicKey.findProgramAddressSync(
      [Buffer.from('round'), epochBytes],
      programId
  );
  return address;
}

export function getBullOrBearAddress(programId: PublicKey) {
  const [address] = PublicKey.findProgramAddressSync(
      [Buffer.from('bullorbear')],
      programId
  );
  return address;
}

// This is a helper function to get the program ID for the Bullorbear program depending on the cluster.
export function getBullorbearProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Bullorbear program on devnet and testnet.
      return new PublicKey('F4Cu5nYYQYJU9qdqyDcZsMbadcNeADDZTqD9AnN12DFK')
    case 'mainnet-beta':
    default:
      return BULLORBEAR_PROGRAM_ID
  }
}
