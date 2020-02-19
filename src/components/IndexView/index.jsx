import React from "react";
import {
  faGamepad,
  faVolumeUp,
  faEnvelope,
} from "@fortawesome/free-solid-svg-icons";
import { faGithub, faSoundcloud } from "@fortawesome/free-brands-svg-icons";

import Image from "../Image";
import ExternalLink from "../ExternalLink";
import ContactItem from "../ContactItem";
import { Wrapper, Box, Spacer } from "./IndexView.style";

const IndexPage = () => (
  <Wrapper>
    <Box center>
      <h1>Allan Legemaate</h1>
    </Box>

    <Box>
      <Image />
    </Box>

    <Box>
      <h2>Who am I?</h2>
      <p>
        I am a 3rd year Computer Science student studying at Queen's University.
      </p>

      <h2>What do I do?</h2>
      <p>
        I am currently working as a full stack developer at Innovasium Digital.
        In my spare time I make games and produce music.
      </p>

      <h2>Contact me</h2>
      <ContactItem text="alegemaate@gmail.com" icon={faEnvelope} />
      <Spacer />
      <Spacer />
    </Box>

    <Box>
      <h2>My Pages</h2>
      <ExternalLink
        text="github"
        icon={faGithub}
        src="https://github.com/alegemaate"
      />
      <Spacer />
      <ExternalLink
        text="soundcloud"
        icon={faSoundcloud}
        src="https://soundcloud.com/allan-legemaate"
      />
      <Spacer />
      <ExternalLink
        text="freesound"
        icon={faVolumeUp}
        src="https://freesound.org/people/alegemaate/"
      />
      <Spacer />
      <ExternalLink
        text="adsgames"
        icon={faGamepad}
        src="https://adsgames.net"
      />
      <Spacer />
    </Box>
  </Wrapper>
);

export default IndexPage;
