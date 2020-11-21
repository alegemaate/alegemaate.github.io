module.exports = {
  env: {
    browser: true,
    es6: true,
    node: true,
  },
  extends: [
    "eslint:all",
    "plugin:@typescript-eslint/all",
    "plugin:react/all",
    "prettier",
    "prettier/react",
    "prettier/@typescript-eslint",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 2018,
    project: "tsconfig.json",
    sourceType: "module",
  },
  plugins: ["@typescript-eslint", "react"],
  root: true,
  rules: {
    "@typescript-eslint/naming-convention": "off",
    "@typescript-eslint/no-type-alias": "off",
    "@typescript-eslint/prefer-readonly-parameter-types": "off",
    camelcase: "off",
    "no-ternary": "off",
    "one-var": "off",
    "react/function-component-definition": [
      "error",
      { namedComponents: "arrow-function" },
    ],
    "react/jsx-curly-brace-presence": "off",
    "react/jsx-filename-extension": ["error", { extensions: [".tsx", ".jsx"] }],
    "react/prop-types": "off",
    "sort-imports": "off",
  },
  settings: {
    react: {
      version: "detect",
    },
  },
};
