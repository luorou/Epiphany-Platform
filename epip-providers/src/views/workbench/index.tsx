import { Breadcrumb, Layout, Menu } from "antd";
import IndexMenu from "@/components/indexMenu";
import TopMenu from "@/components/topMenu";
import { useState } from "react";
import { Outlet } from "react-router-dom";
import "@/views/workbench/index.scss";
const { Header, Content, Footer, Sider } = Layout;

function WorkBenchView() {
  //
  const [collapsed, setCollapsed] = useState(false);

  return (
    <Layout >
      <TopMenu />
      <Layout style={{ minHeight: "100vh"} } >
        <Sider
          collapsible={ false}
          collapsed={collapsed}
          onCollapse={(value) => setCollapsed(value)}
          className="sider-container"
        >
          <div className="logo" />
          <IndexMenu />
        </Sider>
        <Layout className="right-side-layout">
          <Header  style={{ padding: 0 }} className="right-side-layout">
            <Breadcrumb style={{ margin: "16px 16px 0" }}>
              
            </Breadcrumb>
          </Header>
          <Content
            style={{ margin: "16px 16px 0" }}
          >
            <Outlet></Outlet>
          </Content>
          <Footer style={{ textAlign: "center" }}>
            Ant Design ©2018 Created by Ant UED
          </Footer>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default WorkBenchView;
