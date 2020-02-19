import React from "react";
import PropTypes from "prop-types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { LinkStyle, LinkText, LinkIcon } from "./ExternalLink.style";

const ExternalLink = ({ text = "Link", icon, src = "" }) => (
  <a href={src} target="_blank" rel="noopener noreferrer">
    <LinkStyle>
      <LinkText>/{text}/</LinkText>
      <LinkIcon>
        <FontAwesomeIcon icon={icon} />
      </LinkIcon>
    </LinkStyle>
  </a>
);

ExternalLink.propTypes = {
  text: PropTypes.string,
  icon: PropTypes.object.isRequired,
  src: PropTypes.string,
};

export default ExternalLink;
