import { FC } from "react";
import { Button, Space } from "antd";
import {
  DeleteOutlined,
  EditOutlined,
  PlusCircleOutlined,
} from "@ant-design/icons";
import { useSelector, useDispatch } from "react-redux";
import { subject } from "@/pages/Menu/conf";
import { useFgDisabled, useIdUiConf, useSelectedNode } from "../hooks";
import { remove } from "../store";

const NodeAction: FC = () => {
  const dispatch = useDispatch();
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const selectedNode = useSelectedNode();

  const handleToAdd = () => {
    subject.publish({
      topic: "toAdd",
      producerId: idUiConf!,
      data: { treeSelectedNode: selectedNode },
    });
    subject.publish({
      topic: "/page/change",
      producerId: idUiConf!,
      data: "form",
    });
  };

  const handleToEdit = () => {
    subject.publish({
      topic: "toEdit",
      producerId: idUiConf!,
      data: { selectedRow: selectedNode },
    });
    subject.publish({
      topic: "/page/change",
      producerId: idUiConf!,
      data: "form",
    });
  };

  const handleToDelete = () => {
    if (!(selectedNode?.children && selectedNode.children.length > 0)) {
      dispatch(remove(selectedNode));
    }
  };

  return (
    <>
      <Space size={"small"}>
        <Button onClick={handleToAdd} type={"default"} disabled={fgDisabled}>
          <PlusCircleOutlined />
        </Button>
        <Button
          onClick={handleToEdit}
          type={"default"}
          disabled={!selectedNode || fgDisabled}
        >
          <EditOutlined />
        </Button>
        <Button
          onClick={handleToDelete}
          type={"default"}
          disabled={
            !selectedNode ||
            (selectedNode?.children && selectedNode.children.length > 0) ||
            fgDisabled
          }
        >
          <DeleteOutlined />
        </Button>
      </Space>
    </>
  );
};

export default NodeAction;
