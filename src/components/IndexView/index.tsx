import React from "react";
import { StaticImage } from "gatsby-plugin-image";

import { Wrapper, Box } from "./IndexView.style";
import { Pages } from "./Pages";
import { About } from "./About";

const IndexPage: React.FC = () => (
  <Wrapper>
    <Box center>
      <h1>{"Allan Legemaate"}</h1>
    </Box>

    <Box noPad>
      <StaticImage
        alt="Image of Allan Legemaate"
        src="../../images/me.jpg"
        title="Yes, I know this theme looks like nofrills(tm)"
      />
    </Box>

    <About />

    <Pages />
  </Wrapper>
);

export default IndexPage;
