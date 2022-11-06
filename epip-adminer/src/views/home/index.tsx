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
import { useNavigate, Outlet } from "react-router-dom";
import IndexMenu from "@/components/indexMenu/index"

const { Header, Content, Footer, Sider } = Layout;



function Home() {
  //
  const [collapsed, setCollapsed] = useState(false);
  

  return (
    <Layout style={{ minHeight: "100vh" }}>
      //
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={(value) => setCollapsed(value)}
      >
        <div className="logo" />
        <IndexMenu />
      </Sider>
      //
      <Layout className="site-layout">
        <Header className="site-layout-background" style={{ padding: 0 }}>
          <Breadcrumb style={{ margin: "16px 16px 0" }}>
            <Breadcrumb.Item>User</Breadcrumb.Item>
            <Breadcrumb.Item>Bill</Breadcrumb.Item>
          </Breadcrumb>
        </Header>
        <Content
          style={{ margin: "16px 16px 0" }}
          className="site-layout-background"
        >
          <Outlet></Outlet>
        </Content>
        <Footer style={{ textAlign: "center" }}>
          Ant Design ©2018 Created by Ant UED
        </Footer>
      </Layout>
    </Layout>
  );
}

export default Home;
