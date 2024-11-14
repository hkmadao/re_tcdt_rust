import { FC, Key, useEffect, useState } from 'react';
import { Button, Modal, } from 'antd';
import { Observer, TMessage } from '@/util/observer';
import { subject, actionTableConf, } from '../../conf';
import { TTree } from '@/models';

const TableToolBar: FC<{
  idLayout: string
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}> = ({ idLayout, fgDisabled }) => {
  const [componentFgDiabled, setComponentFgDiabled] = useState<boolean>(fgDisabled);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [multiButtonContent, setMultiButtonContent] = useState<string>('多选');
  const [nodeTreeData, setTreeNodeData] = useState<TTree>();
  const [selectRows, setSelectRows] = useState<any[]>([]);
  const [rowSelectionType, setRowSelectionType] = useState<
    'checkbox' | 'radio'
  >('radio');

  useEffect(() => {
    setComponentFgDiabled(fgDisabled);
  }, [fgDisabled]);

  useEffect(() => {
    const treeNodeObserver: Observer = {
      topic: 'treeNodeSelected',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        (async () => {
          if (!message) {
            return;
          }
          const nodeData: TTree = message?.data as TTree;
          setTreeNodeData(nodeData);
        })();
      },
    };
    subject.subscribe(treeNodeObserver);

    const treeNodeCancelObserver: Observer = {
      topic: 'treeSelectCancel',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idLayout)) {
            return;
          }
          setTreeNodeData(undefined);
        })();
      },
    };
    subject.subscribe(treeNodeCancelObserver);

    const selectRowsObserver: Observer = {
      topic: 'selectRows',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idLayout)) {
          return;
        }
        setSelectRows(message.data);
      },
    };
    subject.subscribe(selectRowsObserver);

    const listReloadObserver: Observer = {
      topic: 'listReload',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idLayout)) {
          return;
        }
      },
    };
    subject.subscribe(listReloadObserver);

    //销毁观察者
    return () => {
      subject.unsubsribe(treeNodeObserver);
      subject.unsubsribe(treeNodeCancelObserver);
      subject.unsubsribe(selectRowsObserver);
      subject.unsubsribe(listReloadObserver);
    };
  }, []);

  const handleToAdd = () => {
    subject.publish({
      topic: 'toAdd',
      producerId: idLayout,
      data: { treeSelectedNode: nodeTreeData },
    });
    subject.publish({
      topic: '/page/change',
      producerId: idLayout,
      data: 'form',
    });
  };

  const handleToEdit = () => {
    subject.publish({
      topic: 'toEdit',
      producerId: idLayout,
      data: { treeSelectedNode: nodeTreeData, selectedRow: selectRows[0] },
    });
    subject.publish({
      topic: '/page/change',
      producerId: idLayout,
      data: 'form',
    });
  };

  const handleRowsDelete = () => {
    setIsModalVisible(true);
  };

  const handleRowSelectType = () => {
    if (rowSelectionType !== 'checkbox') {
      setMultiButtonContent('取消多选');
      subject.publish({
        topic: 'checkbox',
        producerId: idLayout,
        data: undefined,
      });
      setRowSelectionType('checkbox');
      setSelectRows([]);
      return;
    }
    subject.publish({
      topic: 'radio',
      producerId: idLayout,
      data: undefined,
    });
    setRowSelectionType('radio');
    setMultiButtonContent('多选');
    setSelectRows([]);
  };

  const handleOk = () => {
    subject.publish({
      topic: 'deletes',
      producerId: idLayout,
      data: undefined,
    });
    setIsModalVisible(false);
  };

  const handleCancel = () => {
    setIsModalVisible(false);
  };

  const handleReflesh = () => {
    subject.publish({
      topic: 'reflesh',
      producerId: idLayout,
      data: undefined,
    });
  };

  return (
    <>
      <div
{%- raw %}
        style={{
          display: 'flex',
          flex: '0 1 auto',
          gap: actionTableConf?.gap ?? '10px',
          justifyContent: actionTableConf?.justifyContent ?? 'start',
          flexWrap: 'wrap',
        }}
{%- endraw %}
      >
{%- if rootInfo.vButtonJson and rootInfo.vButtonJson.buttons is iterable %}
  {%- for buttonConf in rootInfo.vButtonJson.buttons %}
        <Button
          key={'{{ buttonConf.idButton }}'}
          size={'{{ buttonConf.buttonSize }}'}
          type={'{{ buttonConf.type }}'}
    {%- if buttonConf.disableScript %}
          disabled={{ "{" }}{{ buttonConf.disableScript }}{{ "}" }}
    {%- endif %}
    {%- if buttonConf.hiddenScript %}
          hidden={{ "{" }}{{ buttonConf.hiddenScript }}{{ "}" }}
    {%- endif %}
          onClick={{ "{" }}{{ buttonConf.clickEventName }}{{ "}" }}
        >
          {'{{ buttonConf.label }}'}
        </Button>
  {%- endfor %}
{%- endif %}
      </div>
      <Modal
        title="删除确认"
        open={isModalVisible}
        onOk={handleOk}
        onCancel={handleCancel}
      >
        <p>确定删除所选记录？</p>
      </Modal>
    </>
  );
};

export default TableToolBar;
