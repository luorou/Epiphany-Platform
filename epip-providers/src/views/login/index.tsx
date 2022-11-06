import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Card, Input, Space, Button, Form, message } from "antd";
import bicyclesWindAnimation from "@/assets/lottie/lottie-bicycles-wind-turbines.json";
import { useLottie } from "lottie-react";
import "@/views/login/index.scss";
import { useState } from "react";
import { setToken, setMemberInfo } from "@/store/modules/global/action";
import { LoginResult, LoginReq } from "@/api/login/index";
import { loginApi } from "@/api/login/index";
import { get_member_info } from "@/api/member/index";
import { useNavigate } from "react-router-dom";
import { useDispatch } from "react-redux";

function LoginView() {
  //
  const navigate = useNavigate();
  //
  const dispath = useDispatch();
  //
  const [loading, setLoading] = useState<boolean>(false);
  //
  const options = {
    animationData: bicyclesWindAnimation,
    loop: true,
  };

  //
  const { View } = useLottie(options);

  // 登录
  const onFinish = async (req: LoginReq) => {
    try {
      setLoading(true);
      const { data } = await loginApi(req);
      if (data) {
        dispath(setToken(data));
        getMemberInfo();
        message.success("登录成功！");
        navigate("/home");
      }
    } finally {
      setLoading(false);
    }
  };

  /**
   * 
   */
  const getMemberInfo = async () => {
    const { data } = await get_member_info();
    if (data) {
      console.log(data);
      dispath(setMemberInfo(JSON.stringify(data)));
    }
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log("Failed:", errorInfo);
  };

  return (
    <div className="body-container">
      <div className="animation-container">{View}</div>

      <div className="login-action-container">
        <Form
          name="basic"
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          initialValues={{ remember: true }}
          onFinish={onFinish}
          onFinishFailed={onFinishFailed}
          autoComplete="off"
        >
          <Form.Item
            label="邮箱"
            name="email"
            rules={[{ required: true, message: "请输入邮箱" }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="密码"
            name="password"
            rules={[{ required: true, message: "请输入密码" }]}
          >
            <Input.Password />
          </Form.Item>
          <Form.Item wrapperCol={{ offset: 0, span: 16 }}>
            <Button
              type="primary"
              htmlType="submit"
              shape="round"
              className="button"
            >
              Login
            </Button>
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}

export default LoginView;
