module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['**/*.test.ts'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
  globals: {
    transform: {
      '^.+.ts?$': ['ts-jest', { tsconfig: './tsconfig.json' }],
    },
  },
};
