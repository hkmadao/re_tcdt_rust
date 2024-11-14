import { Spin } from 'antd';
import { FC } from 'react';
import classNames from 'classnames';
import ModuleLayout from './components';
import { useLoadingStatus } from './hooks';
import styles from './index.less';

const MainModule: FC = () => {
  const loadStatus = useLoadingStatus();
  return (
    <>
      <Spin
        spinning={loadStatus === 'loading'}
        wrapperClassName={classNames(styles.spin)}
      >
        <ModuleLayout />
      </Spin>
    </>
  );
};

export default MainModule;
