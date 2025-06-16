module.exports = {
  env: {
    browser: true,
  },
  extends: ["eslint:recommended", "plugin:react/recommended", "prettier"],
  parserOptions: {
    ecmaFeatures: ["jsx"],
    ecmaVersion: "latest",
    sourceType: "module",
  },
  plugins: ["react"],
  rules: {
    "react/prop-types": 0,
    "react/react-in-jsx-scope": 0,
  },
  settings: {
    react: {
      version: "detect",
    },
  },
};
