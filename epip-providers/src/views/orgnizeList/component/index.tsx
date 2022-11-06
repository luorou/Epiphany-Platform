import { OrgnizeInfo } from "@/models";
import { Space } from "antd";
import { type } from "os";

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
      let statusTxt = "";
      //0:审核；1:正常；-1；审核不通过；-99:删除
      if (props.status == 0) {
        statusTxt = "未审核，请等待";
      } else if (props.status == 1) {
        statusTxt = "进入工作台";
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
};

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
};