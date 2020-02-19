import React from "react";
import { StaticQuery, graphql } from "gatsby";
import Img from "gatsby-image";

const Image = () => (
  <StaticQuery
    query={graphql`
      query {
        placeholderImage: file(relativePath: { eq: "me.jpg" }) {
          childImageSharp {
            fluid(maxWidth: 400) {
              ...GatsbyImageSharpFluid
            }
          }
        }
      }
    `}
    render={data => (
      <Img
        fluid={data.placeholderImage.childImageSharp.fluid}
        title="Yes, I know this theme looks like nofrills(tm)"
        alt="Image of Allan Legemaate"
      />
    )}
  />
);

export default Image;
