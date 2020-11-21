import * as React from "react";
import type { FC } from "react";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { IconProp } from "@fortawesome/fontawesome-svg-core";

import { Item, Icon } from "./ContactItem.style";

const ContactItem: FC<{ text: string; icon: IconProp }> = ({
  text = "",
  icon,
}) => (
  <Item>
    <Icon>
      <FontAwesomeIcon icon={icon} />
    </Icon>
    <p>{text}</p>
  </Item>
);

export default ContactItem;
