import { FC, useEffect, useState } from 'react';
import { Button, Modal } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { Observer, TMessage } from '@/util/observer';
import { subject, actionTableConf } from '../../../../conf';
import {
  useFgDisabled,
  useIdUiConf,
  useRowSelectionType,
  useSelectRows,
  useTreeNodeData,
} from '../hooks';
import { TTree } from '@/models';
import { actions } from '../store';

const SearchAreaComponent: FC<{}> = ({}) => {
  const [isModalVisible, setIsModalVisible] = useState(false);
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const nodeTreeData = useTreeNodeData();
  const selectRows = useSelectRows();
  const rowSelectionType = useRowSelectionType();
  const dispatch = useDispatch();

  useEffect(() => {
    if (!idUiConf) {
      return;
    }
    
    const treeNodeObserver: Observer = {
      topic: 'treeNodeSelected',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        (async () => {
          if (!message) {
            return;
          }
          const nodeData: TTree = message?.data as TTree;
          dispatch(actions.setTreeNodeData(nodeData));
        })();
      },
    };
    subject.subscribe(treeNodeObserver);

    const treeNodeCancelObserver: Observer = {
      topic: 'treeSelectCancel',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf!)) {
            return;
          }
          dispatch(actions.cancelTreeNodeData());
        })();
      },
    };
    subject.subscribe(treeNodeCancelObserver);

    const selectRowsObserver: Observer = {
      topic: 'selectRows',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf!)) {
          return;
        }
        dispatch(actions.setSelectRows(message.data));
      },
    };
    subject.subscribe(selectRowsObserver);

    const listReloadObserver: Observer = {
      topic: 'listReload',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf!)) {
          return;
        }
        dispatch(actions.setSelectRows([]));
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
  }, [idUiConf]);

  const handleToAdd = () => {
    subject.publish({
      topic: 'toAdd',
      producerId: idUiConf!,
      data: { treeSelectedNode: nodeTreeData! },
    });
    subject.publish({
      topic: '/page/change',
      producerId: idUiConf!,
      data: 'form',
    });
  };

  const handleToEdit = () => {
    subject.publish({
      topic: 'toEdit',
      producerId: idUiConf!,
      data: { treeSelectedNode: nodeTreeData, selectedRow: selectRows[0] },
    });
    subject.publish({
      topic: '/page/change',
      producerId: idUiConf!,
      data: 'form',
    });
  };

  const handleRowsDelete = () => {
    setIsModalVisible(true);
  };

  const handleRowSelectType = () => {
    if (rowSelectionType !== 'checkbox') {
      subject.publish({
        topic: 'checkbox',
        producerId: idUiConf!,
        data: undefined,
      });
      dispatch(actions.setRowSelectionType('checkbox'));
      return;
    }
    subject.publish({
      topic: 'radio',
      producerId: idUiConf!,
      data: undefined,
    });
    dispatch(actions.setRowSelectionType('radio'));
  };

  const handleOk = () => {
    subject.publish({
      topic: 'deletes',
      producerId: idUiConf!,
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
      producerId: idUiConf!,
      data: undefined,
    });
  };

{%- if rootInfo.vButtonJson and rootInfo.vButtonJson.buttons is iterable %}
  {%- for buttonConf in rootInfo.vButtonJson.buttons %}
    {%- if buttonConf.clickEventName == "handleToAdd" %}

    {%- elif buttonConf.clickEventName == "handleToEdit" %}

    {%- elif buttonConf.clickEventName == "handleRowsDelete" %}

    {%- elif buttonConf.clickEventName == "handleRowSelectType" %}

    {%- elif buttonConf.clickEventName == "handleCancel" %}

    {%- elif buttonConf.clickEventName == "handleReflesh" %}

    {%- else %}
  const {{ buttonConf.clickEventName }} = () => {
    // TODO
  };
    {%- endif %}
  {%- endfor %}
{%- endif %}
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
          {{"{ "}}{{ buttonConf.nameScript }}{{" }"}}
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

export default SearchAreaComponent;
