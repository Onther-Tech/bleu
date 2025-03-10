import React, { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import InfoCard from '../../../components/InfoCard';
import { timeSince } from '../../../utils/time';
import {
  Link,
  Table,
  TableBody,
  TableFooter,
  TableHead,
  TablePagination,
  TableRow,
  TableCell,
  Typography,
} from '@mui/material';
import { useRecoilState } from 'recoil';
import { options, state as _state } from './state';
import { L2BlockLink, L1TransactionLink } from '../../../components/Link';
import { useTranslation } from 'react-i18next';
import { api } from '../../../utils/urlResolver';

function BlockList() {
  const { t } = useTranslation('', { useSuspense: false });
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);
  const { search } = useLocation();
  const isState = new URLSearchParams(search).get('isState') === 'true';

  const reload = (count: number, page: number) => {
    if (!isState) {
      (async () => {
        const res = await fetch(api('/tx-batch', undefined, { count: count, page: page }));
        const json = await res.json();
        setState(json);
      })();
    } else {
      (async () => {
        const res = await fetch(api('/stateroot-batch', undefined, { count: count, page: page }));
        const json = await res.json();
        setState(json);
      })();
    }
  };
  const handleChangePage = (event: any, newPage: any) => {
    reload(opts.numRows, newPage + 1);
  };
  const handleChangeRowsPerPage = (event: any) => {
    const page = Math.floor(((state.page_info.page - 1) * opts.numRows) / +event.target.value);
    setOpts({
      ...opts,
      numRows: +event.target.value,
    });
    reload(+event.target.value, page + 1);
  };
  const toggleTimestamp = () => {
    setOpts({
      ...opts,
      datetime: !opts.datetime,
    });
  };

  useEffect(() => {
    reload(opts.numRows, 1);
  }, []);

  return (
    <InfoCard title={!isState ? 'Tx Batches' : 'State Batches'} sx={{ height: '' }}>
      <Table size="small">
        <TableHead sx={{ bgcolor: 'background.default' }}>
          <TableRow>
            <TableCell>{t('Tx Batch')}</TableCell>
            <TableCell>
              <Link sx={{ fontWeight: 'inherit' }} component="button" underline="none" onClick={toggleTimestamp}>
                {opts.datetime ? t('Date Time (UTC)') : t('Age')}
              </Link>
            </TableCell>
            <TableCell>{t('Batch Size')}</TableCell>
            <TableCell>{t('L1 Tx Hash')}</TableCell>
            <TableCell>{t('Prev Total Elements')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {state.records
            ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>
                    <L2BlockLink blockNumber={row.batch_index} isState={isState} />
                  </TableCell>
                  <TableCell>
                    <Typography noWrap={true}>
                      {opts.datetime
                        ? new Date(+row.batch_timestamp * 1000).toLocaleString()
                        : timeSince(row.batch_timestamp)}
                    </Typography>
                  </TableCell>
                  <TableCell>{row.batch_size}</TableCell>
                  <TableCell>
                    <L1TransactionLink hash={row.l1_tx_hash} />
                  </TableCell>
                  <TableCell>{row.previous_total_elements}</TableCell>
                </TableRow>
              ))
            : null}
        </TableBody>
        <TableFooter>
          <TableRow>
            {state.page_info ? (
              <TablePagination
                rowsPerPageOptions={[10, 25, 50, 100]}
                colSpan={5}
                count={state.page_info.total_count}
                rowsPerPage={opts.numRows}
                page={state.page_info.page - 1}
                SelectProps={{
                  inputProps: {
                    'aria-label': 'rows per page',
                  },
                  native: true,
                }}
                onPageChange={handleChangePage}
                onRowsPerPageChange={handleChangeRowsPerPage}
                showFirstButton={true}
                showLastButton={true}
                sx={{ borderBottom: 'none' }}
              />
            ) : null}
          </TableRow>
        </TableFooter>
      </Table>
    </InfoCard>
  );
}

export default BlockList;
