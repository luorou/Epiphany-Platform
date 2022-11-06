import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Card, Input, Space, Button, Form, message } from "antd";
import bicyclesWindAnimation from "@/assets/lottie/lottie-bicycles-wind-turbines.json";
import { useLottie } from "lottie-react";
import "./index.scss";
import { useState } from "react";
import { setToken } from "@/store/modules/global/action";
import { registerApi, RegisterReq } from "@/api/register/index";
import { useNavigate } from "react-router-dom";
import { useDispatch } from "react-redux";

function RegisterView() {
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
  const onFinish = async (req: RegisterReq) => {
    try {
      if (req.password !== req.submitPassword) {
        message.success("请确认两次密码输入是否一致");
      }
      setLoading(true);
      const { data } = await registerApi(req);
      if (data) {
        console.log(data);
        message.success("注册成功！");
        navigate("/login");
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

        <Card
          style={{ marginTop: 0 }}
          type="inner"
          title="Inner Card title"
          className="login-action-container"
        >
          <Form
            name="register"
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
            <Form.Item
              label="确认密码"
              name="submitPassword"
              rules={[{ required: true, message: "请确认密码" }]}
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
                注册
              </Button>
            </Form.Item>
          </Form>
        </Card>
    </div>
  );
}

export default RegisterView;
