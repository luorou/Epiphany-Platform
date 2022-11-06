import { Space, Button, Layout } from "antd";
const { Header } = Layout;
import store from "@/store/index";
import "./index.scss";
import { useNavigate } from "react-router-dom";
import { MemberInfo } from "@/models/index"

function RightMenu() {
  //
  const navigate = useNavigate();
  //
  const openLogin = () => {
    navigate("/login");
  };
  //
  const openRegister = () => {
    navigate("/register");
  };
  //
  const openWorkbench = () => {
    navigate("/orgnize_list");
    
  };
  let token: string|null = store.getState().global.token;
  if (token) {
    return (
      <Space className="right-menu">
        <a
          className="work-bench"
          style={{ marginLeft: "16px" }}
          onClick={openWorkbench}
        >
          工作台
        </a>
      </Space>
    );
  } else {
    return (
      <Space className="right-menu">
        <Button shape="round" onClick={openLogin}>
          登录
        </Button>
        <Button shape="round" onClick={openRegister}>
          注册
        </Button>
      </Space>
    );
  }
}

const TopMenu = function topMenu() {
  //
  const navigate = useNavigate();
  //
  const openCommunity = () => {
    navigate("/community");
  };

  //
  const openDocument = () => {
    navigate("/documention");
  };

  return (
    <Header className="header-menu-container" style={{ padding: 0 }}>
      <Space>
        <div className="platform-title">Epiphany 开放平台</div>
        <a
          className="tab-title"
          style={{ marginLeft: "64px" }}
          onClick={openCommunity}
        >
          社区
        </a>
        <a
          className="tab-title"
          style={{ marginLeft: "16px" }}
          onClick={openDocument}
        >
          文档
        </a>
        <RightMenu />
      </Space>
    </Header>
  );
};

export default TopMenu;
