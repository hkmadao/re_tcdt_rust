import { FC, Key, useEffect, useRef, useState, useMemo, } from 'react';
import { Button, TreeProps, Input, Tree, InputRef, Space, Checkbox, } from 'antd';
import { CloseCircleFilled, CloseOutlined, ReloadOutlined, SearchOutlined } from '@ant-design/icons';
import { useSelector, useDispatch } from 'react-redux';
import { TTree } from '@/models';
import { Observer, TMessage } from '@/util/observer';
import { subject } from '../../../conf';
import {
  useTreeData,
  useIdUiConf,
  useExpandedKeys,
  useSelectedKeys,
  useFoundKeys,
  useFgDisabled,
  useFgInnerDisabled,
} from '../hooks';
import { actions, fetchTree } from '../store';
import NodeAction from './action';

type TOprationLayout = {};

const LeftTreeLayout: FC<TOprationLayout> = () => {
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const fgInnerDisabled = useFgInnerDisabled();
  const treeDatas = useTreeData();
  const searchRef = useRef<InputRef>(null);
  const expandedKeys = useExpandedKeys();
  const selectedKeys = useSelectedKeys();
  const foundKeys = useFoundKeys();
  const [searchValue, setSearchValue] = useState<string>();

  const dispatch = useDispatch();

  const fgComponentDisabled = useMemo(() => fgDisabled || fgInnerDisabled, [fgDisabled, fgInnerDisabled]);

  useEffect(() => {
    if (!idUiConf) {
      return;
    }
    const searchObserver: Observer = {
      topic: 'search',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          // dispatch(actions.cancelSelectedNode());
        })();
      },
    };
    subject.subscribe(searchObserver);

    const saveObserver: Observer = {
      topic: 'addSuccess',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf)) {
            return;
          }
          //获取树信息
          dispatch(fetchTree());
        })();
      },
    };
    subject.subscribe(saveObserver);

    //获取树信息
    dispatch(fetchTree());

    //销毁观察者
    return () => {
      subject.unsubsribe(searchObserver);
      subject.unsubsribe(saveObserver);
    };
  }, [idUiConf]);

  const onReflesh = () => {
    dispatch(fetchTree());
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key !== "Enter") {
      return;
    }
    const value = searchRef.current?.input?.value || '';
    dispatch(actions.searchTreeNode(value));
  };

  const handleClear = () => {
    if (searchRef.current?.input?.value) {
      let value = '';
      setSearchValue(value);
      dispatch(actions.searchTreeNode(value));
    }
  };

  const handleSearch = () => {
    const value = searchRef.current?.input?.value || '';
    dispatch(actions.searchTreeNode(value));
  };

  const handleToggleInnerDisabled = () => {
    dispatch(actions.setFgInnerDisabled(!fgInnerDisabled));
    if (!fgInnerDisabled) {
      dispatch(actions.cancelSelectedNode());
    }
  };

  const handleChange = (e: any) => {
    setSearchValue(e.currentTarget.value);
  };

  const handleClick = (e: any, node: any) => {
  };

  const handleDoubleClick = (e: any, node: any) => {
  };

  const onSelect = (keys: Key[], { node, nativeEvent }: { node: TTree; nativeEvent: MouseEvent; }) => {
    //只有第一次点击执行事件，防止连续多次执行
    if (nativeEvent.detail === 1) {
      if (keys && keys.length > 0) {
        dispatch(actions.setSelectedNode({ keys, node }));
      }
    }
  };

  const onExpand = (
    keys: Key[],
    { node }: { node: TTree; expanded: boolean },
  ) => {
    dispatch(actions.toggleExpand(node.key));
  };

  /** 树主配置 */
  const treeConfig: TreeProps<TTree & any> = {
    disabled: fgComponentDisabled,
    treeData: treeDatas,
    expandedKeys,
    selectedKeys: selectedKeys,
    showIcon: false,
    showLine: true,
    onSelect,
    onExpand,
    onClick: handleClick,
    onDoubleClick: handleDoubleClick,
    titleRender: (nodeData) => {
      return (
        <span
{%- raw %}
          style={{
            display: 'inline-block',
            minWidth: '20px',
            whiteSpace: 'nowrap',
            color: foundKeys.includes(nodeData.key) ? 'red' : undefined,
          }}
{%- endraw %}
        >
          {nodeData.title}
        </span>
      );
    },
  };

  return (
    <>
      <div
{%- raw %}
        style={{
          display: 'flex',
          flex: 'auto',
          minWidth: 200,
        }}
{%- endraw %}
      >
        <div
{%- raw %}
          style={{
            display: 'flex',
            flex: 'auto',
            flexDirection: 'column',
            backgroundColor: 'white',
            gap: '10px',
          }}
{%- endraw %}
        >
          <Space size={'small'}>
            <Input
              ref={searchRef}
              value={searchValue}
              onChange={handleChange}
              readOnly={fgComponentDisabled}
              onKeyDown={handleKeyDown}
              suffix={
                <Space direction="horizontal" size={2}>
                  {searchValue ? <CloseCircleFilled style={{ "{{" }} color: 'rgb(191 191 191)' {{ "}}" }} onClick={handleClear} /> : ''}
                </Space>
              } />
            <Button onClick={handleSearch} type={'primary'} disabled={fgComponentDisabled}>
              <SearchOutlined />
            </Button>
            <Button onClick={onReflesh} type={'primary'} disabled={fgComponentDisabled}>
              <ReloadOutlined />
            </Button>
          </Space>
          <div style={{ "{{" }} display: 'flex', justifyContent: 'end', gap: '10px' {{ "}}" }}>
            {'禁用状态:'}
            <Checkbox
              checked={fgInnerDisabled}
              disabled={fgDisabled}
              onClick={handleToggleInnerDisabled}
            ></Checkbox>
          </div>
          {/* <NodeAction /> */}
          <Tree {...treeConfig} />
        </div>
      </div>
    </>
  );
};

export default LeftTreeLayout;
