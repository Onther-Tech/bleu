import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import { RecoilRoot } from 'recoil';
import Box from '@mui/material/Box';
import { ThemeProvider } from '@mui/material/styles';
import theme from './styles/theme';
import './App.css';
import './i18n';
import Header from './components/Header';
import Footer from './components/Footer';
import MainPage from './pages/MainPage';
import AccountPage from './pages/AccountPage';
import BlockDetailsPage from './pages/BlockDetailsPage/BlockDetailsPage';
import BlocksPage from './pages/BlocksPage';
import L1L2TransactionsPage from './pages/L1L2TransactionsPage';
import TransactionDetailsPage from './pages/TransactionDetailsPage';
import TransactionsPage from './pages/TransactionsPage';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'space-between',
  bgcolor: 'background.default',
  minHeight: '100vh',
};

const main: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
};

function App() {
  return (
    <RecoilRoot>
      <ThemeProvider theme={theme()}>
        <Box sx={root}>
          <Box sx={main}>
            <Header />
            <Router>
              <Switch>
                <Route path="/txs">
                  <TransactionsPage />
                </Route>
                <Route path="/blocks">
                  <BlocksPage />
                </Route>
                <Route path={'/l1l2txs'}>
                  <L1L2TransactionsPage />
                </Route>
                <Route path="/account/:address">
                  <AccountPage />
                </Route>
                <Route path={'/block/:blockNumber'}>
                  <BlockDetailsPage />
                </Route>
                <Route path={'/tx/:txHash'}>
                  <TransactionDetailsPage />
                </Route>
                <Route path="/">
                  <MainPage />
                </Route>
              </Switch>
            </Router>
          </Box>
          <Footer />
        </Box>
      </ThemeProvider>
    </RecoilRoot>
  );
}

export default App;
