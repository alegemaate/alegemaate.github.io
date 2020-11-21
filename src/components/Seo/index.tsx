import React from "react";
import type { FC } from "react";

import Helmet from "react-helmet";
import { useStaticQuery, graphql } from "gatsby";

type Meta = React.DetailedHTMLProps<
  React.MetaHTMLAttributes<HTMLMetaElement>,
  HTMLMetaElement
>;

interface MetaQuery {
  site: {
    siteMetadata: { title: string; description: string; author: string };
  };
}

const generateMeta = ({
  description,
  title,
  author,
  meta,
}: {
  description: string;
  title: string;
  author: string;
  meta: Meta[];
}): Meta[] => {
  return [
    {
      content: description,
      name: "description",
    },
    {
      content: title,
      property: "og:title",
    },
    {
      content: description,
      property: "og:description",
    },
    {
      content: "website",
      property: "og:type",
    },
    {
      content: "summary",
      name: "twitter:card",
    },
    {
      content: author,
      name: "twitter:creator",
    },
    {
      content: title,
      name: "twitter:title",
    },
    {
      content: description,
      name: "twitter:description",
    },
    ...meta,
  ];
};

const Seo: FC<{
  description?: string;
  lang?: string;
  meta?: Meta[];
  title?: string;
}> = ({ description = "", lang = "en", meta = [], title }) => {
  const { site } = useStaticQuery<MetaQuery>(
    // eslint-disable-next-line @typescript-eslint/no-confusing-void-expression
    graphql`
      query {
        site {
          siteMetadata {
            title
            description
            author
          }
        }
      }
    `
  );

  return (
    <Helmet
      htmlAttributes={{ lang }}
      meta={generateMeta({
        author: site.siteMetadata.author,
        description: description || site.siteMetadata.description,
        meta,
        title: site.siteMetadata.author,
      })}
      title={title}
      titleTemplate={`%s | ${site.siteMetadata.title}`}
    />
  );
};

export default Seo;
