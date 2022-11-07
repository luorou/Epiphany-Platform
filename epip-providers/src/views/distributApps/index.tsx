import { Button, Select, Space } from "antd";
import Search from "antd/lib/input/Search";
import "@/views/distributApps/index.scss"

const selectOptions = [
  {
    value: "100",
    label: "全部",
  },
  //0:未提交; 1:审核中: 2待发布;3:已发布;-1:审核不通过;-99:已下架
  {
    value: "0",
    label: "未提交",
  },
  {
    value: "1",
    label: "审核中",
  },
  {
    value: "2",
    label: "待发布",
  },
  {
    value: "3",
    label: "已发布",
  },
  {
    value: "-1",
    label: "审核不通过",
  },
  {
    value: "-99",
    label: "已下架",
  },
];

function DistributAppsView() {
  const onSearch = (value: string) => console.log(value);
  const onSelectChange = (value: string) => {
    console.log(`selected ${value}`);
  };
  return (
    <div className="body-layout">
      <Space className="header-action-layout">
        <Search
          placeholder="请输入完整应用名称或包名"
          onSearch={onSearch}
          enterButton
          style={{ width: 270 }}
        />
        <Select
          defaultValue="100"
          style={{ width: 120 }}
          onChange={onSelectChange}
          options={selectOptions}
        />
        <Button
          type="primary"
          htmlType="submit"
          shape="round"
          style={{ width: 120 }}
          className="create-button"
        >
          创建应用
        </Button>
      </Space>
    </div>
  );
}

export default DistributAppsView;
