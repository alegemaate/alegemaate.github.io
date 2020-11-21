import React from "react";
import type { FC } from "react";

import "./layout.css";

const Layout: FC = ({ children }) => (
  <div
    style={{
      margin: `0 auto`,
      maxWidth: 960,
      padding: `0px 1.0875rem 1.45rem`,
      paddingTop: 0,
    }}
  >
    <main>{children}</main>
    <footer>{`© ${new Date().getFullYear()}, Allan Legemaate`}</footer>
  </div>
);

export default Layout;
