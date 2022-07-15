import { selector } from 'recoil';
import { api } from '../../../../utils/urlResolver';
import { options } from '../state';

export interface Log {
  address: string;
  block_hash: string;
  block_number: string;
  data: string;
  log_index: string;
  removed: boolean;
  topics: string[];
  tx_hash: string;
  tx_index: string;
}

export const state = selector<Log[]>({
  key: 'TransactionPageLogsState',
  get: async ({ get }) => {
    const opts = get(options);
    if (opts.txHash.length === 0) {
      return;
    }
    if (opts.txHash.startsWith('0x')) {
      const res = await fetch(api('/tx/logs/hash', opts.txHash));
      return await res.json();
    } else {
    }
  },
});
