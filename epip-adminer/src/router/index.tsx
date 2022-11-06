import { lazy } from "react";
import { Navigate } from "react-router-dom";
import { RouteObject } from "./interface/index";
import withLoadingElement from "./lazyLoad";
const Home = lazy(() => import("../views/home"));
const Login = lazy(() => import("../views/login"));
const Dashboard = lazy(() => import("../views/dashboard"));
const CateApps = lazy(() => import("../views/cateApps"));
const CateGames = lazy(() => import("../views/cateGames"));
const Adminer = lazy(() => import("../views/adminer"));
const Orgnizes = lazy(() => import("../views/orgnizes"));
const Members = lazy(() => import("../views/members"));
const SoftApps = lazy(() => import("../views/softApps"));
//

const routes: RouteObject[] = [
  {
    path: "/",
    element: <Navigate to="/index" />,
  },
  {
    path: "/index",
    element: <Home />,
    children: [
      {
        path: "/index/dashboard",
        element: withLoadingElement(<Dashboard />),
      },
      {
        path: "/index/category/apps",
        element: withLoadingElement(<CateApps />),
      },
      {
        path: "/index/category/games",
        element: withLoadingElement(<CateGames />),
      },
      {
        path: "/index/users/adminer",
        element: withLoadingElement(<Adminer />),
      },
      {
        path: "/index/users/organize",
        element: withLoadingElement(<Orgnizes />),
      },
      {
        path: "/index/users/members",
        element: withLoadingElement(<Members />),
      },
      {
        path: "/index/software",
        element: withLoadingElement(<SoftApps />),
      },
    ],
  },
  {
    path: "/login",
    element: withLoadingElement(<Login />),
    meta: {
      requiresAuth: false,
    },
  },
];

export default routes;
