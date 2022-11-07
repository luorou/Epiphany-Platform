import { OrgnizeInfo } from "@/models";
import { Space } from "antd";
import { useNavigate } from "react-router-dom";

export type StatusProps = {
  record: OrgnizeInfo;
};

export function StatusColumn(props: OrgnizeInfo) {
  let statusTxt = "";
  //0:审核；1:正常；-1；审核不通过；-99:删除
  if (props.status == 0) {
    statusTxt = "未审核";
  } else if (props.status == 1) {
    statusTxt = "审核通过";
  } else if (props.status == -1) {
    statusTxt = "拒绝";
  } else if (props.status == -99) {
    statusTxt = "已删除";
  }
  return (
    <Space size="middle">
      <>{statusTxt}</>
    </Space>
  );
}

export function ActionColmun(props: OrgnizeInfo) {
  //
  const navigate = useNavigate();
  //
  const actionClick = () => {
    if (props.status == 1) {
      navigate("/workbench");
    } else if (props.status == -1) {
          
    }
  };
  //
  let statusTxt = "";
  //0:审核；1:正常；-1；审核不通过；-99:删除
  if (props.status == 0) {
    statusTxt = "--";
  } else if (props.status == 1) {
    statusTxt = "进入工作台";
  } else if (props.status == -1) {
    statusTxt = "编辑";
  } else if (props.status == -99) {
    statusTxt = "--";
  }
  return (
    <Space size="middle">
      <a onClick={actionClick}>{statusTxt}</a>
    </Space>
  );
}

export function TypeColmun(props: OrgnizeInfo) {
  let statusTxt = "";
  //0:审核；1:正常；-1；审核不通过；-99:删除
  if (props.organize_type == 0) {
    statusTxt = "个人";
  } else if (props.status == 1) {
    statusTxt = "公司";
  }
  return (
    <Space size="middle">
      <>{statusTxt}</>
    </Space>
  );
}
