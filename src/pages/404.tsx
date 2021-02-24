import React from "react";
import type { FC } from "react";

import Layout from "../components/Layout";
import Seo from "../components/Seo";

const NotFoundPage: FC = () => (
  <Layout>
    <Seo title="Page Not Found" />
    <h1>{"Page not found!"}</h1>
    <p>{"You just hit a route that doesn't exist... the sadness."}</p>
  </Layout>
);

export default NotFoundPage;
