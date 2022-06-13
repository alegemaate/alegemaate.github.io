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
    "gatsby-plugin-image",
    "gatsby-plugin-sharp",
    "gatsby-transformer-sharp",
    {
      options: {
        background_color: "#d9a057",
        display: "standalone",
        icon: "src/images/favicon.png",
        name: "Allan Legemaate's Site",
        short_name: "Allan's Site",
        start_url: "/",
        theme_color: "#d9a057",
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
