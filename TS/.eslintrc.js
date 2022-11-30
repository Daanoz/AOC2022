module.exports = {
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint'],
    extends: ['plugin:@typescript-eslint/recommended'],
    rules: {
        '@typescript-eslint/no-empty-function': ['off'],
        '@typescript-eslint/no-non-null-assertion': ['off'],
        semi: ['error', 'never'],
        quotes: ['error', 'single']
    }
}