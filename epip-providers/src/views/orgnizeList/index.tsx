import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Card, Input, Space, Button, Form, message, Layout } from "antd";
import bicyclesWindAnimation from "@/assets/lottie/lottie-bicycles-wind-turbines.json";
import { useLottie } from "lottie-react";
import "@/views/orgnizeList/index.scss";
import { useEffect, useState } from "react";
import { setToken, setMemberInfo } from "@/store/modules/global/action";
import { get_organize_info } from "@/api/orgnize/index";
import { useNavigate } from "react-router-dom";
import { useDispatch } from "react-redux";
import { OrgnizeInfo } from "@/models/index";
import Table, { ColumnsType } from "antd/lib/table";
import { Content } from "antd/lib/layout/layout";
import { ActionColmun, StatusColumn, TypeColmun } from "./component";

const columns: ColumnsType<OrgnizeInfo> = [
  {
    title: "名称",
    dataIndex: "name",
    key: "name",
    width: "25%",
  },
  {
    title: "组织类型",
    dataIndex: "organize_type",
    key: "organize_type",
    width: "20%",
    render: (_, record) => {
      return <TypeColmun {...record} />;
    },
  },
  {
    title: "创建时间",
    dataIndex: "create_time",
    key: "create_time",
    width: "25%",
  },
  {
    title: "状态",
    dataIndex: "status",
    key: "organize_id",
    width: "25%",
    render: (_, record) => {
      return <StatusColumn {...record} />;
    },
  },
  {
    title: "操作",
    dataIndex: "status",
    key: "organize_id",
    width: "10%",
    render: (_, record) => {
      return <ActionColmun {...record} />;
    },
  },
];

function OrgnizeListView() {
  //
  const [loading, setLoading] = useState<boolean>(false);
  //
  const [orgnizeList, setOrgnizeList] = useState<OrgnizeInfo[]>([]);
  //
  const navigate = useNavigate();
  //
  const dispath = useDispatch();

  useEffect(() => {
    const fetchData = async () => {
      const { data } = await get_organize_info();
      if (data) {
        // setOrgnizeList(data);
      }
    };
    fetchData();
  }, []);
  //
  const create_orgnize_click = () => {
    navigate("/create_orgnize");
  };

  return (
    <div className="body-container">
      <div className="title-container" onClick={create_orgnize_click}>
        添加组织
      </div>
      <Table
        rowKey={(record) => record.organize_id}
        bordered
        pagination={false}
        dataSource={orgnizeList}
        columns={columns}
      />
    </div>
  );
}

export default OrgnizeListView;
