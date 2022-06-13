import React from "react";

import { faEnvelope } from "@fortawesome/free-solid-svg-icons";

import ContactItem from "../ContactItem";

import { Box, Spacer } from "./IndexView.style";

const calcBirthday = (): number => {
  const birthday = new Date("1997-04-22");
  const ageDifMs = Date.now() - birthday.getTime();
  const ageDate = new Date(ageDifMs);
  return Math.abs(ageDate.getUTCFullYear() - 1970);
};

export const About: React.FC = () => (
  <Box>
    <h2>{"Who am I?"}</h2>
    <p>
      {`I am a ${calcBirthday()} year old Computer Science graduate from Queen's University.`}
    </p>

    <h2>{"What do I do?"}</h2>
    <p>
      {
        "I am currently working as a full stack developer at Adeptmind. In my spare time I make games and produce music."
      }
    </p>

    <h2>{"Contact me"}</h2>
    <ContactItem icon={faEnvelope} text="alegemaate@gmail.com" />
    <Spacer />
    <Spacer />
  </Box>
);
