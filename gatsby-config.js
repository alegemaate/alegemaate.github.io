module.exports = {
  plugins: [
    "gatsby-plugin-react-helmet",
    {
      options: {
        name: "images",
        path: `${__dirname}/src/images`,
      },
      resolve: "gatsby-source-filesystem",
    },
    "gatsby-transformer-sharp",
    "gatsby-plugin-sharp",
    {
      options: {
        background_color: "#dbb700",
        display: "standalone",
        icon: "src/images/favicon.png",
        name: "Allan Legemaate's Site",
        short_name: "Allan's Site",
        start_url: "/",
        theme_color: "#dbb700",
      },
      resolve: "gatsby-plugin-manifest",
    },
    {
      options: {
        allExtensions: true,
        isTSX: true,
        jsxPragma: "jsx",
      },
      resolve: "gatsby-plugin-typescript",
    },
  ],
  siteMetadata: {
    author: "alegemaate@gmail.com",
    description: "Just a basic page",
    title: "Allan Legemaate's Page",
  },
};
