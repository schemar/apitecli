module.exports = {
  env: {
    es6: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'airbnb-base',
    'plugin:@typescript-eslint/recommended'
  ],
  globals: {
    Atomics: 'readonly',
    SharedArrayBuffer: 'readonly',
  },
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2018,
    sourceType: 'module',
  },
  plugins: [
    '@typescript-eslint',
  ],
  rules: {
    "import/extensions": [
      "error",
      "ignorePackages",
      {
        "ts": "never"
      }
    ]
  },
  settings: {
    "import/resolver": {
      "node": {
          "paths": ["src"],
          "extensions": [".ts"]
      }
    }
  }
};
