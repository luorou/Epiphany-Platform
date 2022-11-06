import type { RadioChangeEvent } from "antd";
import { Radio, List } from "antd";
import React, { useState, useEffect } from "react";
import { list_category, CategryResult } from "@/api/category/index";

function View() {
  const [value, setValue] = useState(1);
  const [cates, setCates] = useState<CategryResult[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      const { data } = await list_category({ cate_type: 1, level: 2 });
      if (data) {
        setCates(data);
      }
    };
    fetchData();
  }, []);

  return (
    <List
      bordered
      dataSource={cates}
      renderItem={(item) => (
        <List.Item>
          <div className="level-1-item">{item.name}</div>
          <List
            grid={{ gutter: 16, column: 6 }}
            bordered
            dataSource={item.child}
            renderItem={(item) => <List.Item><div className="level-2-item">{item.name}</div></List.Item>}
          />
        </List.Item>
      )}
    />
  );
}

export default View;
