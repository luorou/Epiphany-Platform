import {
  AppstoreOutlined,
  PieChartOutlined,
  TeamOutlined,
  UserOutlined,
} from "@ant-design/icons";
import type { MenuProps } from "antd";
import { Breadcrumb, Layout, Menu } from "antd";
import React, { useState } from "react";
import { useNavigate } from "react-router-dom";

type MenuItem = Required<MenuProps>["items"][number];

function getItem(
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[]
): MenuItem {
  return {
    key,
    icon,
    children,
    label,
  } as MenuItem;
}

const items: MenuItem[] = [
  {
    label: "应用分发",
    key: "distribution",
    icon: <AppstoreOutlined />,
    children: [
      {
        label: "游戏",
        key: "/workbench/distribut/games",
      },
      {
        label: "应用",
        key: "/workbench/distribut/apps",
      },
    ],
  },
];

function ComponentLayout() {
  //
  const navigate = useNavigate();
  //
  const menuClick = (e: { key: string }) => {
    navigate(e.key);
  };

  return (
    <Menu
      theme="light"
      defaultSelectedKeys={["/workbench/distribut/apps"]}
      mode="inline"
      items={items}
      onClick={menuClick}
      // openKeys={openKeys}
    />
  );
}

export default ComponentLayout;
