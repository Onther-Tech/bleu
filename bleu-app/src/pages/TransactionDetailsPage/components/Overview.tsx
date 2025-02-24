import React from 'react';
import { Box, Table, TableBody, TableCell, TableRow, TextareaAutosize, Typography } from '@mui/material';
import { useRecoilValueLoadable } from 'recoil';
import { state } from './Overview/state';
import { L1TransactionLink, L2AddressLink, L2BlockLink } from '../../../components/Link';
import { timeSince } from '../../../utils/time';
import { toEther, txFee } from '../../../utils/ethUtils';

function parseInputData(input: string): string {
  if (input.length < 10) {
    return input;
  }
  let ret = `MethodID: ${input.slice(0, 10)}`;
  let i = 0;
  input = input.slice(10);
  while (input.length > 0) {
    ret += `\n[${i}]` + (i < 10 ? ' ' : '') + ` ${input.slice(0, 64)}`;
    input = input.slice(64);
    i += 1;
  }
  return ret;
}

function Overview() {
  const stateLoadable = useRecoilValueLoadable(state);

  return (
    <React.Fragment>
      {stateLoadable.state === 'hasValue' && stateLoadable.contents ? (
        <Table>
          <TableBody>
            <TableRow>
              <TableCell>
                <Typography>Transaction Hash</Typography>
              </TableCell>
              <TableCell>
                <Typography>{stateLoadable.contents.tx_ext.tx.hash}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>Status</Typography>
              </TableCell>
              <TableCell>
                <Typography>{+stateLoadable.contents.tx_ext.state ? 'Success' : 'Failed'}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>Transaction Index</Typography>
              </TableCell>
              <TableCell>
                <Typography>{stateLoadable.contents.tx_ext.tx.index}</Typography>
              </TableCell>
            </TableRow>
            {stateLoadable.contents.tx_ext.l1_tx_batch_index ? (
              <TableRow>
                <TableCell>
                  <Typography>L1 Txn Batch Index</Typography>
                </TableCell>
                <TableCell>
                  <L2BlockLink blockNumber={stateLoadable.contents.tx_ext.l1_tx_batch_index} isState={false} />
                </TableCell>
              </TableRow>
            ) : null}
            {stateLoadable.contents.tx_ext.l1_submission_tx_hash ? (
              <TableRow>
                <TableCell>
                  <Typography>L1 Submission Tx Hash</Typography>
                </TableCell>
                <TableCell>
                  <L1TransactionLink hash={stateLoadable.contents.tx_ext.l1_submission_tx_hash} />
                </TableCell>
              </TableRow>
            ) : null}
            {stateLoadable.contents.l1_origin_tx_hash ? (
              <TableRow>
                <TableCell>
                  <Typography>L1 Origin Tx Hash</Typography>
                </TableCell>
                <TableCell>
                  <L1TransactionLink hash={stateLoadable.contents.l1_origin_tx_hash} />
                </TableCell>
              </TableRow>
            ) : null}
            {stateLoadable.contents.tx_ext.l1_state_batch_index ? (
              <TableRow>
                <TableCell>
                  <Typography>L1 State Batch Index</Typography>
                </TableCell>
                <TableCell>
                  <L2BlockLink blockNumber={stateLoadable.contents.tx_ext.l1_state_batch_index} isState={true} />
                </TableCell>
              </TableRow>
            ) : null}
            {stateLoadable.contents.tx_ext.l1_state_root_submission_tx_hash ? (
              <TableRow>
                <TableCell>
                  <Typography>L1 State Root Submission Tx Hash</Typography>
                </TableCell>
                <TableCell>
                  <L1TransactionLink hash={stateLoadable.contents.tx_ext.l1_state_root_submission_tx_hash} />
                </TableCell>
              </TableRow>
            ) : null}
            <TableRow>
              <TableCell>
                <Typography>Timestamp</Typography>
              </TableCell>
              <TableCell>
                <Box sx={{ display: 'flex', gap: '8px' }}>
                  <Typography>{timeSince(stateLoadable.contents.tx_ext.tx.l1_timestamp)}</Typography>
                  <Typography>
                    ({new Date(+stateLoadable.contents.tx_ext.tx.l1_timestamp * 1000).toLocaleString()})
                  </Typography>
                </Box>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>From</Typography>
              </TableCell>
              <TableCell>
                <L2AddressLink address={stateLoadable.contents.tx_ext.tx.from_address} />
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>To</Typography>
              </TableCell>
              <TableCell>
                {stateLoadable.contents.tx_ext.tx.to_address ? (
                  <L2AddressLink address={stateLoadable.contents.tx_ext.tx.to_address} />
                ) : (
                  <React.Fragment>
                    [Contract <L2AddressLink address={stateLoadable.contents.tx_ext.contract_address} /> Created]
                  </React.Fragment>
                )}
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>Value</Typography>
              </TableCell>
              <TableCell>
                <Typography>{toEther(stateLoadable.contents.tx_ext.tx.value)} Ether</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>Transaction Fee</Typography>
              </TableCell>
              <TableCell>
                <Typography>
                  {txFee(stateLoadable.contents.tx_ext.gas_used, stateLoadable.contents.tx_ext.tx.gas_price)} Ether
                </Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>L2 Gas Price</Typography>
              </TableCell>
              <TableCell>
                <Typography>{(+stateLoadable.contents.tx_ext.tx.gas_price).toLocaleString()} Wei</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>L2 Gas Used</Typography>
              </TableCell>
              <TableCell>
                <Typography>{(+stateLoadable.contents.tx_ext.gas_used).toLocaleString()}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>L1 Gas Used</Typography>
              </TableCell>
              <TableCell>
                <Typography>{(+stateLoadable.contents.tx_ext.l1_gas_used).toLocaleString()}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>L1 Gas Price</Typography>
              </TableCell>
              <TableCell>
                <Typography>{(+stateLoadable.contents.tx_ext.l1_gas_price).toLocaleString()} Wei</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>L1 Fee Scalar</Typography>
              </TableCell>
              <TableCell>
                <Typography>{(+stateLoadable.contents.tx_ext.l1_fee_scalar).toLocaleString()}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell>
                <Typography>Nonce</Typography>
              </TableCell>
              <TableCell>
                <Typography>{stateLoadable.contents.tx_ext.tx.nonce}</Typography>
              </TableCell>
            </TableRow>
            <TableRow>
              <TableCell sx={{ borderBottom: 'none' }}>
                <Typography>Input Data</Typography>
              </TableCell>
              <TableCell sx={{ borderBottom: 'none' }}>
                <TextareaAutosize
                  style={{
                    width: '100%',
                    resize: 'vertical',
                    backgroundColor: '#f5f5f5',
                    color: '#74838e',
                  }}
                  aria-label="transaction-details-input-data"
                  readOnly
                  defaultValue={
                    stateLoadable.contents.tx_ext.tx.to_address
                      ? parseInputData(stateLoadable.contents.tx_ext.tx.tx_input)
                      : stateLoadable.contents.tx_ext.tx.tx_input
                  }
                />
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      ) : null}
    </React.Fragment>
  );
}

export default Overview;
