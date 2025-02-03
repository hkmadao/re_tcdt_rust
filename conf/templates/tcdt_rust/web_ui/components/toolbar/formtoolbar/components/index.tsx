import { FC, useEffect } from 'react';
import { Button } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { Observer, TMessage } from '@/util/observer';
import { subject, actionFormConf } from '../../../../conf';
import { useFgAdd, useFgDisabled, useIdUiConf } from '../hooks';
import { actions } from '../store';

const FromToolBarComponent: FC<{}> = ({}) => {
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const fgAdd = useFgAdd();
  const dispatch = useDispatch();

  useEffect(() => {
    if (!idUiConf) {
      return;
    }
    
    const toAddObserver: Observer = {
      topic: 'toAdd',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf!)) {
          return;
        }
        dispatch(actions.setFgAdd(true));
      },
    };
    subject.subscribe(toAddObserver);

    const toEditObserver: Observer = {
      topic: 'toEdit',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf!)) {
          return;
        }
        dispatch(actions.setFgAdd(false));
      },
    };
    subject.subscribe(toEditObserver);

    //销毁观察者
    return () => {
      subject.unsubsribe(toAddObserver);
      subject.unsubsribe(toEditObserver);
    };
  }, [idUiConf]);

  const handleSave = () => {
    if (fgAdd) {
      subject.publish({
        topic: 'add',
        producerId: idUiConf!,
        data: undefined,
      });
    } else {
      subject.publish({
        topic: 'edit',
        producerId: idUiConf!,
        data: undefined,
      });
    }
  };

  const handleAddAgain = () => {
    subject.publish({
      topic: 'addAgain',
      producerId: idUiConf!,
      data: undefined,
    });
  };

  const handleCancel = () => {
    subject.publish({
      topic: 'cancel',
      producerId: idUiConf!,
      data: undefined,
    });
  };

  const handleReflesh = () => {
    subject.publish({
      topic: 'detailReflesh',
      producerId: idUiConf!,
      data: undefined,
    });
  };

{%- if rootInfo.buttonJson and rootInfo.buttonJson.buttons is iterable %}
  {%- for buttonConf in rootInfo.buttonJson.buttons %}
    {%- if buttonConf.clickEventName == "handleSave" %}

    {%- elif buttonConf.clickEventName == "handleAddAgain" %}

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
            gap: actionFormConf?.gap ?? '10px',
            justifyContent: actionFormConf?.justifyContent ?? 'start',
            flexWrap: 'wrap',
          }}
{%- endraw %}
        >
{%- if rootInfo.buttonJson and rootInfo.buttonJson.buttons is iterable %}
  {%- for buttonConf in rootInfo.buttonJson.buttons %}
          <Button
            key={'{{ buttonConf.idButton }}'}
            size={'{{ buttonConf.buttonSize }}'}
            type={'{{ buttonConf.type }}'}
    {%- if buttonConf.disableScript %}
            disabled={{ "{" }}{{ buttonConf.disableScript }}{{ "}" }}
    {%- endif %}
            onClick={{ "{" }}{{ buttonConf.clickEventName }}{{ "}" }}
    {%- if buttonConf.hiddenScript %}
            hidden={{ "{" }}{{ buttonConf.hiddenScript }}{{ "}" }}
    {%- endif %}
          >
            {{"{ "}}{{ buttonConf.nameScript }}{{" }"}}
          </Button>
  {%- endfor %}
{%- endif %}
        </div>
    </>
  );
};

export default FromToolBarComponent;
