
import { lazy } from "react";
import { Navigate } from "react-router-dom";
import { RouteObject } from "./interface/index";
import withLoadingElement from "./lazyLoad";
const Home = lazy(() => import("../views/home"));
const Login = lazy(() => import("../views/login"));
const Register = lazy(() => import("../views/register"));
const WorkBench = lazy(() => import("../views/workbench"));
const Community = lazy(() => import("../views/community"));
const Documention = lazy(() => import("../views/documention"));
const EditOrgnize = lazy(() => import("../views/editOrgnize"));
const OrgnizeList = lazy(() => import("../views/orgnizeList"));
const CreateOrgnize = lazy(() => import("../views/createOrgnize"));
const DistributAppsView = lazy(() => import("../views/distributApps"));
const DistributGamesView = lazy(() => import("../views/distributGames"));

//

const routes: RouteObject[] = [
  {
    path: "/",
    element: <Navigate to="/home" />,
  },
  {
    path: "/home",
    element: <Home />,
  },
  {
    path: "/login",
    element: withLoadingElement(<Login />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/register",
    element: withLoadingElement(<Register />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/workbench",
    element: withLoadingElement(<WorkBench />),
    meta: {
      requiresAuth: true,
    },
    children: [
      {
        path: "/workbench/distribut/apps",
        element: withLoadingElement(<DistributAppsView />),
      },
      {
        path: "/workbench/distribut/games",
        element: withLoadingElement(<DistributGamesView />),
      },
    ],
  },
  {
    path: "/community",
    element: withLoadingElement(<Community />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/documention",
    element: withLoadingElement(<Documention />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/edit_orgnize",
    element: withLoadingElement(<EditOrgnize />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/orgnize_list",
    element: withLoadingElement(<OrgnizeList />),
    meta: {
      requiresAuth: false,
    },
  },
  {
    path: "/create_orgnize",
    element: withLoadingElement(<CreateOrgnize />),
    meta: {
      requiresAuth: false,
    },
  },
];

export default routes;
