import { FC, Key, memo, useEffect, useState, useRef } from 'react';
import { Table, Tabs } from 'antd';
import { DownOutlined } from '@ant-design/icons';
import { useSelector, useDispatch } from 'react-redux';
import moment from 'moment';
import { Observer, TMessage } from '@/util/observer';
import { useMainTableColumns } from '../hooks/columns';
import { subject,} from '../../../../conf';
import styles from './styles.less';
import { batchRemove, fetchByTreeNode, pageChange, reflesh, search } from '../store';
import { useStoreData, useIdUiConf, } from '../hooks';
import { actions } from '../store';
import { 
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';

{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
const MainTableLayout: FC = () => {
  const idUiConf = useIdUiConf();
  const columns = useMainTableColumns();
  const [rowSelectionType, setRowSelectionType] = useState<
    'checkbox' | 'radio'
  >('radio');
  const tableStore = useStoreData();
  const dispatch = useDispatch();

  useEffect(() => {
{%- if not rootInfo.tJson %}
    if (!tableStore.tableData || tableStore.tableData.length === 0) {
      dispatch(reflesh());
    }
{%- endif %}
  },[]);

  useEffect(() => {
    if (!idUiConf) {
      return;
    }
    const treeNodeObserver: Observer = {
      topic: 'treeNodeSelected',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(fetchByTreeNode(message));
        })();
      },
    };
    subject.subscribe(treeNodeObserver);

    const treeNodeCancelObserver: Observer = {
      topic: 'treeSelectCancel',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(actions.setSelectedTreeNode(undefined));
        })();
      },
    };
    subject.subscribe(treeNodeCancelObserver);

    const searchObserver: Observer = {
      topic: 'search',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(search(message));
        })();
      },
    };
    subject.subscribe(searchObserver);

    const checkboxObserver: Observer = {
      topic: 'checkbox',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(actions.setSelectedRowKeys([]));
          setRowSelectionType('checkbox');
        })();
      },
    };
    subject.subscribe(checkboxObserver);

    const radioObserver: Observer = {
      topic: 'radio',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(actions.setSelectedRowKeys([]));
          setRowSelectionType('radio');
        })();
      },
    };
    subject.subscribe(radioObserver);

    const addSuccessObserver: Observer = {
      topic: 'addSuccess',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          const addAfterData = message.data;
          dispatch(actions.addNewRecords(addAfterData));
          setRowSelectionType('radio');
        })();
      },
    };
    subject.subscribe(addSuccessObserver);

    const updateSuccessObserver: Observer = {
      topic: 'updateSuccess',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          const updateAfterData = message.data;
          dispatch(actions.updateRecord(updateAfterData));
          setRowSelectionType('radio');
        })();
      },
    };
    subject.subscribe(updateSuccessObserver);

    const deletesObserver: Observer = {
      topic: 'deletes',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(batchRemove(message));
        })();
      },
    };
    subject.subscribe(deletesObserver);

    const refleshObserver: Observer = {
      topic: 'reflesh',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(reflesh());
        })();
      },
    };
    subject.subscribe(refleshObserver);

    //销毁观察者
    return () => {
      subject.unsubsribe(treeNodeObserver);
      subject.unsubsribe(treeNodeCancelObserver);
      subject.unsubsribe(searchObserver);
      subject.unsubsribe(checkboxObserver);
      subject.unsubsribe(radioObserver);
      subject.unsubsribe(addSuccessObserver);
      subject.unsubsribe(updateSuccessObserver);
      subject.unsubsribe(deletesObserver);
      subject.unsubsribe(refleshObserver);
    };
  }, [idUiConf]);

  const onPageChange = (page: number, pageSize: number) => {
    (async () => {
      dispatch(pageChange({ page, pageSize }));
    })();
  };

  const showTotal = (total: number) => {
    return <>总数：{total}</>;
  };

  const onChange = (
    selectedRowKeys: Key[],
    selectedRows: T{{ bt.tabClassName }}[],
  ) => {
  };

  const handleSelect = (
    record: T{{ bt.tabClassName }},
    select: boolean,
    selectedRows: T{{ bt.tabClassName }}[],
  ) => {
    if (rowSelectionType == 'checkbox') {
      const newKeys = selectedRows.map(r => r.{{ bt.mainProperty }}!);
      dispatch(actions.setSelectedRowKeys(newKeys));
    } else {
      dispatch(actions.setSelectedRowKeys([record.{{ bt.mainProperty }}!]));
    }
  }

  const onRow = (record: T{{ bt.tabClassName }}) => {
    return {
      onClick: (event: any) => {
        if (rowSelectionType == 'checkbox') {
          let newKeys = tableStore.selectedRowKeys?.slice() || [];
          if (!newKeys.includes(record.{{ bt.mainProperty }}!)) {
            newKeys?.push(record.{{ bt.mainProperty }}!);
            dispatch(actions.setSelectedRowKeys(newKeys));
          } else {
            newKeys = newKeys?.filter(
              (idKey) => idKey !== record.{{ bt.mainProperty }},
            );
            dispatch(actions.setSelectedRowKeys(newKeys));
          }
          dispatch(dispatch(actions.setSelectedRowKeys(newKeys)));
        } else {
          dispatch(actions.setSelectedRowKeys([record.{{ bt.mainProperty }}!]));
        }
      }, // 点击行
      onDoubleClick: (event: any) => {},
      onContextMenu: (event: any) => {},
      onMouseEnter: (event: any) => {}, // 鼠标移入行
      onMouseLeave: (event: any) => {},
    };
  };

  return (
    <>
      <Table
        size={'small'}
        rowKey={'{{ bt.mainProperty }}'}
        onRow={onRow}
        columns={columns}
        dataSource={tableStore.tableData}
        scroll={{ "{{" }} x: 300, y: 500 {{ "}}" }}
        rowSelection={
          rowSelectionType
            ? {
                type: rowSelectionType,
                onChange: onChange,
                selectedRowKeys: tableStore.selectedRowKeys,
                onSelect: handleSelect,
              }
            : undefined
        }
        pagination={
          {
            total: tableStore.totalCount,
            current: tableStore.pageIndex,
            pageSize: tableStore.pageSize,
            onChange: onPageChange,
            showTotal,
          }
        }
      />
    </>
  );
};

export default MainTableLayout;
  {%- endfor %}
{%- endif %}
