import { FC } from 'react';
import { Provider } from 'react-redux';
import store from './store';
import MainModule from './MainModule';

const Module: FC = () => {
  return (
    <>
      <Provider store={store}>
        <MainModule />
      </Provider>
    </>
  );
};

export default Module;
