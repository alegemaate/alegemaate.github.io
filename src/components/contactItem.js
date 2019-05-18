import React from "react"
import PropTypes from "prop-types"

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"

const ContactItem = ({ text, icon }) => (
  <div className="contactItem">
    <div className="contactItemIcon">
      <FontAwesomeIcon icon={icon} />
    </div>
    <p>{text}</p>
  </div>
)

ContactItem.defaultProps = {
  text: "",
}

ContactItem.propTypes = {
  text: PropTypes.string,
  icon: PropTypes.object.isRequired,
}

export default ContactItem
