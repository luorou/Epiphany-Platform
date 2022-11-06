import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Card, Input, Space, Button, Form, message } from "antd";
import bicyclesWindAnimation from "@/assets/lottie/lottie-bicycles-wind-turbines.json";
import { useLottie } from "lottie-react";
import "@/views/login/index.scss";
import { useState } from "react";
import { setToken } from "@/store/modules/global/action";
import { LoginResult, ReqLoginForm } from "@/api/login/index";
import { loginApi } from "@/api/login/index";
import { useNavigate } from "react-router-dom";
import { useDispatch} from 'react-redux'

function LoginView() {
  //
  const navigate = useNavigate();
  //
  const dispath = useDispatch()
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
  const onFinish = async (loginForm: ReqLoginForm) => {
    try {
      setLoading(true);
      const { data } = await loginApi(loginForm);
      if (data) { 
        console.log(data);
        dispath(setToken(data));
        message.success("登录成功！");
        navigate("/index");
      }
    } finally {
      setLoading(false);
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
            label="Account"
            name="account"
            rules={[{ required: true, message: "Please input your account!" }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="Password"
            name="password"
            rules={[{ required: true, message: "Please input your password!" }]}
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
