import React from "react";

import Layout from "../components/Layout";
import Seo from "../components/Seo";
import IndexView from "../components/IndexView";

const IndexPage: React.FC = () => (
  <Layout>
    <Seo title="About Allan Legemaate" />
    <IndexView />
  </Layout>
);

export default IndexPage;
