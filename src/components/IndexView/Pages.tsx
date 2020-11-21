import React from "react";
import type { FC } from "react";

import { faGamepad, faVolumeUp } from "@fortawesome/free-solid-svg-icons";
import { faGithub, faSoundcloud } from "@fortawesome/free-brands-svg-icons";

import ExternalLink from "../ExternalLink";

import { Box, Spacer } from "./IndexView.style";

export const Pages: FC = () => (
  <Box>
    <h2>{"My Pages"}</h2>
    <ExternalLink
      icon={faGithub}
      src="https://github.com/alegemaate"
      text="github"
    />
    <Spacer />
    <ExternalLink
      icon={faSoundcloud}
      src="https://soundcloud.com/allan-legemaate"
      text="soundcloud"
    />
    <Spacer />
    <ExternalLink
      icon={faVolumeUp}
      src="https://freesound.org/people/alegemaate/"
      text="freesound"
    />
    <Spacer />
    <ExternalLink icon={faGamepad} src="https://adsgames.net" text="adsgames" />
    <Spacer />
  </Box>
);
