import React from "react";
import ReactDOM from "react-dom/client";
import "reset-css";
import "@/assets/styles/global.scss";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
import { Provider } from "react-redux";
import store from "./store";
import AuthRouter from "./router/authRouter";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider store={store}>
      <BrowserRouter>
        <AuthRouter>
          <App />
        </AuthRouter>
      </BrowserRouter>
    </Provider>
  </React.StrictMode>
);
