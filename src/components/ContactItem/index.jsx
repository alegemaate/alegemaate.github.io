import React from "react";
import PropTypes from "prop-types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { Item, Icon, Text } from "./ContactItem.style";

const ContactItem = ({ text = "", icon }) => (
  <Item>
    <Icon>
      <FontAwesomeIcon icon={icon} />
    </Icon>
    <p>{text}</p>
  </Item>
);

ContactItem.propTypes = {
  text: PropTypes.string,
  icon: PropTypes.object.isRequired,
};

export default ContactItem;
