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
        background_color: "#663399",
        display: "minimal-ui",
        icon: "src/images/favicon.png",
        name: "gatsby-starter-default",
        short_name: "starter",
        start_url: "/",
        theme_color: "#663399",
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
