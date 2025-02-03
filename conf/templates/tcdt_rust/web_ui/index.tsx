import { FC } from 'react';
import { Provider } from 'react-redux';
import store from './store';
import MainLayout from './main';

const Module: FC = () => {
  return (
    <>
      <Provider store={store}>
        <MainLayout />
      </Provider>
    </>
  );
};

export default Module;
