import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// 글로벌 스타일
import "./styles/global.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);