import React from "react";
import type { FC } from "react";

import { graphql } from "gatsby";

import { Image } from "../Image";

import { Wrapper, Box } from "./IndexView.style";
import { Pages } from "./Pages";
import { About } from "./About";

const IndexPage: FC = () => (
  <Wrapper>
    <Box center>
      <h1>{"Allan Legemaate"}</h1>
    </Box>

    <Box>
      <Image
        alt="Image of Allan Legemaate"
        title="Yes, I know this theme looks like nofrills(tm)"
      />
    </Box>

    <About />

    <Pages />
  </Wrapper>
);

export default IndexPage;
