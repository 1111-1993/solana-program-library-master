import {Account, Connection} from '@solana/web3.js';

/**
 * Create a new system account and airdrop it some weis
 *
 * @private
 */
export async function newSystemAccountWithAirdrop(
  connection: Connection,
  weis: number = 1,
): Promise<Account> {
  const account = new Account();
  await connection.requestAirdrop(account.publicKey, weis);
  return account;
}
