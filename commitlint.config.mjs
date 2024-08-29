const Configuration = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [2, 'always', [
      'readme',
      'ci',
      'repo',
      'db',
      'grades',
      'graphql',
    ]],
  },
};

export default Configuration;
