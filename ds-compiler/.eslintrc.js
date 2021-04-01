module.exports = {
    root: true,
    env: { es6: true, node: true },
    ignorePatterns: ['dist'],
    extends: ['eslint:recommended', 'prettier'],
    plugins: ['prettier', 'simple-import-sort'],
    rules: {
        'prettier/prettier': 'error',
        'simple-import-sort/imports': 'error',
    },
    overrides: [
        {
            files: ['src/**/*.ts', 'scripts/**/*.ts'],
            parser: '@typescript-eslint/parser',
            parserOptions: {
                sourceType: 'module',
                ecmaVersion: 2019,
                tsconfigRootDir: __dirname,
                project: ['./tsconfig.json'],
            },
            extends: [
                'plugin:@typescript-eslint/recommended',
                'plugin:@typescript-eslint/recommended-requiring-type-checking',
            ],
        },
    ],
};
