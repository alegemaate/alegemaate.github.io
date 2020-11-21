/* eslint-disable @typescript-eslint/no-confusing-void-expression */
import React from "react";
import type { FC } from "react";

import { graphql, useStaticQuery } from "gatsby";
import Img from "gatsby-image";
import type { FluidObject } from "gatsby-image";

interface SharpQuery {
  placeholderImage: {
    childImageSharp: { fluid: FluidObject | FluidObject[] };
  };
}

export const Image: FC<{ alt?: string; title?: string }> = ({
  alt = "",
  title = "",
}) => {
  const data = useStaticQuery<SharpQuery>(graphql`
    query {
      placeholderImage: file(relativePath: { eq: "me.jpg" }) {
        childImageSharp {
          fluid(maxWidth: 400) {
            ...GatsbyImageSharpFluid
          }
        }
      }
    }
  `);

  return (
    <Img
      alt={alt}
      fluid={data.placeholderImage.childImageSharp.fluid}
      title={title}
    />
  );
};
