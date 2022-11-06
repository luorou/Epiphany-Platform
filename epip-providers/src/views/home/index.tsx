import { Space, Button, Layout } from "antd";
import React, { useState } from "react";
import { useNavigate, Outlet } from "react-router-dom";
import "./index.scss";
import TopMenu from '@/components/topMenu/index'

const { Header, Content, Footer, Sider } = Layout;

function Home() {
  //
  const [collapsed, setCollapsed] = useState(false);

  return (
    <Layout style={{ minHeight: "100vh" }}>
      <TopMenu />
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
  );
}

export default Home;
