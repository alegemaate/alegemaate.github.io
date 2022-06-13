import React from "react";

import "./layout.css";

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => (
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
