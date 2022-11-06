import type { RadioChangeEvent } from "antd";
import { Table,Space } from "antd";
import React, { useState, useEffect } from "react";
import { get_orgnize, OrgnizeResult } from "@/api/orgnize/index";
import { ColumnsType } from "antd/lib/table";

const columns: ColumnsType<OrgnizeResult> = [
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
    key: "admin_id",
    render: (_, { status }) => {
      let statusTxt = ''
      //0:审核；1:正常；-1；审核不通过；-99:删除
      if (status == 0) {
        statusTxt = '未审核'
      } else if (status == 1) {
        statusTxt = '审核通过'
      }else if (status == -1) {
        statusTxt = '拒绝'
      }else if (status == -99){
        statusTxt = '已删除'
      }
      return (
        <Space size="middle">
          <a>{ statusTxt}</a>
        </Space>
      );
    },

  },
];
function View() {
  const [value, setValue] = useState(1);
  const [adminders, setAdminders] = useState<OrgnizeResult[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      const { data } = await get_orgnize();
      if (data) {
        setAdminders(data);
      }
    };
    fetchData();
  }, []);

  return <Table dataSource={adminders} columns={columns} />;
}

export default View;
