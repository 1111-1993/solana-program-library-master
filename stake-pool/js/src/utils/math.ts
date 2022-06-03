import BN from 'bn.js';
import { WEIS_PER_GTH } from '@solana/web3.js';

export function gthToWeis(amount: number): number {
  if (isNaN(amount)) return Number(0);
  return Number(amount * WEIS_PER_GTH);
}

export function weisToGth(weis: number | BN): number {
  if (typeof weis === 'number') {
    return Math.abs(weis) / WEIS_PER_GTH;
  }

  let signMultiplier = 1;
  if (weis.isNeg()) {
    signMultiplier = -1;
  }

  const absWeis = weis.abs();
  const weisString = absWeis.toString(10).padStart(10, '0');
  const splitIndex = weisString.length - 9;
  const gthString = weisString.slice(0, splitIndex) + '.' + weisString.slice(splitIndex);
  return signMultiplier * parseFloat(gthString);
}
