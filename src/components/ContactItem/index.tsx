import * as React from "react";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { IconDefinition } from "@fortawesome/fontawesome-svg-core";

import { Item, Icon } from "./ContactItem.style";

interface ContactItemProps {
  text: string;
  icon: IconDefinition;
}

const ContactItem: React.FC<ContactItemProps> = ({ text = "", icon }) => (
  <Item>
    <Icon>
      <FontAwesomeIcon icon={icon} />
    </Icon>
    <p>{text}</p>
  </Item>
);

export default ContactItem;
