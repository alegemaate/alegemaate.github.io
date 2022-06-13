import React from "react";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { LinkStyle, LinkText, LinkIcon } from "./ExternalLink.style";
import type { IconDefinition } from "@fortawesome/fontawesome-svg-core";

interface ExternalLinkProps {
  text: string;
  icon: IconDefinition;
  src: string;
}

const ExternalLink: React.FC<ExternalLinkProps> = ({
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
