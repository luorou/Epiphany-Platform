import React, { lazy } from "react";
import { Spin } from "antd";

function withLoadingElement(element: JSX.Element) {
  return (
    <React.Suspense
      fallback={
        <Spin
          size="large"
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            height: "100%",
          }}
        />
      }
    >
      {element}
    </React.Suspense>
  );
}

export default withLoadingElement;
