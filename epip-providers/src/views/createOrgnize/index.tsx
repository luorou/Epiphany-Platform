import { CreateOrgnizeReq, create_orgnize } from "@/api/orgnize";
import { PlusOutlined } from "@ant-design/icons";
import { Button, Form, Input, message, Radio, Upload } from "antd";
import { useState } from "react";
import "@/views/createOrgnize/index.scss";
import { useNavigate } from "react-router-dom";

function CreateOrgnizeView() {
  //
  const [loading, setLoading] = useState<boolean>(false);
  //
  const navigate = useNavigate();
  //
  const onFinish = async (req: CreateOrgnizeReq) => {
    try {
      setLoading(true);
      req.organize_type = Number(req.organize_type)
      const { data } = await create_orgnize(req);
      if (data) {
        message.success("提交成功！");
        navigate("/home");
      }
    } finally {
      setLoading(false);
    }
  };
  //
  const onFinishFailed = (errorInfo: any) => {
    console.log("Failed:", errorInfo);
  };

  return (
    <div className="body-container">
      <div className="form-container">
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
            label="组织名称"
            name="name"
            rules={[{ required: true, message: "请输入组织名称" }]}
          >
            <Input />
          </Form.Item>

          <Form.Item label="组织类型" name="organize_type">
            <Radio.Group>
              <Radio value="0"> 企业 </Radio>
              <Radio value="1"> 个人 </Radio>
            </Radio.Group>
          </Form.Item>

          <Form.Item
            label="营业执照编号"
            name="business_license_id"
            rules={[{ required: true, message: "请输入营业执照编号" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item label="营业执照" valuePropName="fileList">
            <Upload action="/upload.do" listType="picture-card">
              <div>
                <PlusOutlined />
                <div style={{ marginTop: 8 }}>Upload</div>
              </div>
            </Upload>
          </Form.Item>
          <Form.Item
            label="地址"
            name="address"
            rules={[{ required: true, message: "请输入密码" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item label="logo" valuePropName="fileList">
            <Upload action="/upload.do" listType="picture-card">
              <div>
                <PlusOutlined />
                <div style={{ marginTop: 8 }}>Upload</div>
              </div>
            </Upload>
          </Form.Item>
          <Form.Item
            label="联系电话"
            name="phone"
            rules={[{ required: true, message: "请输入联系电话" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            label="联系邮箱"
            name="link_email"
            rules={[{ required: true, message: "请输入邮箱" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            label="联系QQ"
            name="link_qq"
            rules={[{ required: true, message: "请输入QQ" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item wrapperCol={{ offset: 0, span: 16 }}>
            <Button
              type="primary"
              htmlType="submit"
              shape="round"
              className="button"
            >
              提交
            </Button>
          </Form.Item>
        </Form>
      </div>
    </div>
  );
}

export default CreateOrgnizeView;
