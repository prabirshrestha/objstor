import React from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import loadable from '@loadable/component';
import { ChakraProvider } from '@chakra-ui/react';
import { theme } from './theme';

const HomePage = loadable(() => import('./Pages/HomePage'));
const ChangePasswordPage = loadable(() => import('./Pages/ChangePasswordPage'));

export function App() {
  return (
    <ChakraProvider theme={theme}>
      <Router>
        <Switch>
          <Route path="/changepassword" component={ChangePasswordPage} />
          <Route path="/" component={HomePage} />
        </Switch>
      </Router>
    </ChakraProvider>
  );
}
