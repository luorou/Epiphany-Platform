import { useLocation, Navigate } from "react-router-dom";
import { searchRoute } from "@/utils/utils";
import rootRouter from "./index";
import store from "@/store/index";

//
function AuthRouter(props: { children: JSX.Element }) {
  const { pathname } = useLocation();
  if (
    pathname == "/login" ||
    pathname == "/home" ||
    pathname == "/register" ||
    pathname == "/community" ||
    pathname == "/documention"
  ) {
    return props.children;
  }
  const token = store.getState().global.token;
  if (!token) {
    return <Navigate to="/login" replace />;
  }
  return props.children;
}

export default AuthRouter;
