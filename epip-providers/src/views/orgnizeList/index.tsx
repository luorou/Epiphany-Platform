import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Card, Input, Space, Button, Form, message } from "antd";
import bicyclesWindAnimation from "@/assets/lottie/lottie-bicycles-wind-turbines.json";
import { useLottie } from "lottie-react";
import "@/views/orgnizeList/index.scss";
import { useEffect, useState } from "react";
import { setToken, setMemberInfo } from "@/store/modules/global/action";
import { get_orgnize_list } from "@/api/orgnize/index";
import { useNavigate } from "react-router-dom";
import { useDispatch } from "react-redux";
import { OrgnizeInfo } from "@/models/index";
import Table, { ColumnsType } from "antd/lib/table";
import { Content } from "antd/lib/layout/layout";

const columns: ColumnsType<OrgnizeInfo> = [
  {
    title: "名称",
    dataIndex: "name",
    key: "name",
  },
  {
    title: "创建时间",
    dataIndex: "create_time",
    key: "create_time",
  },
  {
    title: "状态",
    dataIndex: "status",
    key: "organize_id",
    render: (_, { status }) => {
      let statusTxt = "";
      //0:审核；1:正常；-1；审核不通过；-99:删除
      if (status == 0) {
        statusTxt = "未审核";
      } else if (status == 1) {
        statusTxt = "审核通过";
      } else if (status == -1) {
        statusTxt = "拒绝";
      } else if (status == -99) {
        statusTxt = "已删除";
      }
      return (
        <Space size="middle">
          <>{statusTxt}</>
        </Space>
      );
    },
  },
];

function OrgnizeListView() {
  const [orgnizeList, setOrgnizeList] = useState<OrgnizeInfo[]>([]);
  //
  const navigate = useNavigate();
  //
  const dispath = useDispatch();

  useEffect(() => {
    const fetchData = async () => {
      const { data } = await get_orgnize_list();
      if (data) {
        setOrgnizeList(data);
      }
    };
    fetchData();
  }, []);
  //
  const [loading, setLoading] = useState<boolean>(false);

  return (
        <div className="tab-container">
              我的组织
        <Table dataSource={orgnizeList} columns={columns} />
    </div>
  );
}

export default OrgnizeListView;
