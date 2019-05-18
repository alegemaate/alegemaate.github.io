import React from "react"
import PropTypes from "prop-types"

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"

const ExternalLink = ({ text, icon, src }) => (
  <a href={src} target="_blank" rel="noopener noreferrer">
    <div className="externalLink">
      <div className="externalLinkText">/{text}/</div>
      <div className="externalLinkIcon">
        <FontAwesomeIcon icon={icon} />
      </div>
    </div>
  </a>
)

ExternalLink.defaultProps = {
  text: "Link",
  src: "",
}

ExternalLink.propTypes = {
  text: PropTypes.string,
  icon: PropTypes.object.isRequired,
  src: PropTypes.string,
}

export default ExternalLink
