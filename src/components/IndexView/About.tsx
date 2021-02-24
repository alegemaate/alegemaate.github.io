import React from "react";
import type { FC } from "react";

import { faEnvelope } from "@fortawesome/free-solid-svg-icons";

import ContactItem from "../ContactItem";

import { Box, Spacer } from "./IndexView.style";

export const About: FC = () => (
  <Box>
    <h2>{"Who am I?"}</h2>
    <p>
      {
        "I am a 5rd year Computer Science student studying at Queen's University."
      }
    </p>

    <h2>{"What do I do?"}</h2>
    <p>
      {
        "I am currently working as a full stack developer at Innovasium Digital. In my spare time I make games and produce music."
      }
    </p>

    <h2>{"Contact me"}</h2>
    <ContactItem icon={faEnvelope} text="alegemaate@gmail.com" />
    <Spacer />
    <Spacer />
  </Box>
);
