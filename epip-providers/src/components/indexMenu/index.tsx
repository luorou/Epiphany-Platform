import {
  DesktopOutlined,
  FileOutlined,
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
    label: "Dashboard",
    key: "/index/dashboard",
    icon: <PieChartOutlined />,
  },
  {
    label: "Category",
    key: "Category",
    icon: <UserOutlined />,
    children: [
      {
        label: "Game",
        key: "/index/category/games",
      },
      {
        label: "Apps",
        key: "/index/category/apps",
      },
    ],
  },
  {
    label: "Users",
    key: "Users",
    icon: <TeamOutlined />,
    children: [
      {
        label: "Adaminer",
        key: "/index/users/adminer",
      },
      {
        label: "Organize",
        key: "/index/users/organize",
      },
      {
        label: "Members",
        key: "/index/users/members",
      },
    ],
  },
  {
    label: "Software",
    key: "/index/software",
    icon: <PieChartOutlined />,
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
      theme="dark"
      defaultSelectedKeys={["/index/dashboard"]}
      mode="inline"
      items={items}
      onClick={menuClick}
      // openKeys={openKeys}
    />
  );
}

export default ComponentLayout;
