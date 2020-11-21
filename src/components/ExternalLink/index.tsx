import React from "react";
import type { FC } from "react";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { IconProp } from "@fortawesome/fontawesome-svg-core";

import { LinkStyle, LinkText, LinkIcon } from "./ExternalLink.style";

const ExternalLink: FC<{ text: string; icon: IconProp; src: string }> = ({
  text = "Link",
  icon,
  src = "",
}) => (
  <LinkStyle href={src} rel="noopener noreferrer" target="_blank">
    <LinkText>{`/${text}/`}</LinkText>
    <LinkIcon>
      <FontAwesomeIcon icon={icon} />
    </LinkIcon>
  </LinkStyle>
);

export default ExternalLink;
