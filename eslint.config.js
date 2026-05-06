import js from '@eslint/js';
import prettier from 'eslint-config-prettier';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import pluginVue from 'eslint-plugin-vue';
import tseslint from 'typescript-eslint';
import vueParser from 'vue-eslint-parser';

export default [
  {
    env: {
      browser: true,
      es2021: true,
    },
  },
  {
    ignores: ['src-tauri/**', 'vite-env.d.ts', 'node_modules/**', 'dist/**', 'coverage/**'],
  },

  /*
  -------------------------
  BASE JS
  -------------------------
  */
  js.configs.recommended,

  /*
  -------------------------
  TYPESCRIPT
  -------------------------
  */
  ...tseslint.configs.recommended,

  /*
  -------------------------
  VUE
  -------------------------
  */
  ...pluginVue.configs['flat/recommended'],

  /*
  -------------------------
  GLOBAL FILES
  -------------------------
  */
  {
    files: ['**/*.{js,mjs,cjs,ts,vue}'],

    languageOptions: {
      parser: vueParser,

      parserOptions: {
        parser: tseslint.parser,
        ecmaVersion: 'latest',
        sourceType: 'module',
        extraFileExtensions: ['.vue'],
      },
    },

    plugins: {
      '@typescript-eslint': tseslint.plugin,
      vue: pluginVue,
      'simple-import-sort': simpleImportSort,
    },

    rules: {
      /*
      =====================================
      IMPORTS
      =====================================
      */
      'simple-import-sort/imports': 'error',
      'simple-import-sort/exports': 'error',

      /*
      =====================================
      TYPESCRIPT STRICT
      =====================================
      */

      // запрет any
      '@typescript-eslint/no-explicit-any': 'error',

      // обязательные return types
      '@typescript-eslint/explicit-function-return-type': [
        'error',
        {
          allowExpressions: true,
          allowTypedFunctionExpressions: true,
        },
      ],

      // unused vars
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
        },
      ],

      // import type {}
      '@typescript-eslint/consistent-type-imports': [
        'error',
        {
          prefer: 'type-imports',
        },
      ],

      // ts-ignore warn
      '@typescript-eslint/ban-ts-comment': 'warn',

      /*
      =====================================
      VUE BLOCK ORDER
      =====================================
      */

      // template -> script -> style
      'vue/block-order': [
        'error',
        {
          order: ['template', 'script', 'style'],
        },
      ],

      /*
      =====================================
      VUE COMPONENT INTERNAL ORDER
      =====================================
      */

      'vue/order-in-components': [
        'error',
        {
          order: ['name', 'components', 'props', 'emits', 'setup', 'computed', 'methods'],
        },
      ],

      /*
      =====================================
      SCRIPT SETUP MACROS ORDER
      =====================================
      */

      'vue/define-macros-order': [
        'error',
        {
          order: ['defineOptions', 'defineProps', 'defineEmits', 'defineSlots'],
        },
      ],

      /*
      =====================================
      CUSTOM COMPONENTS IN TEMPLATE
      kebab-case only
      =====================================
      */

      'vue/component-name-in-template-casing': [
        'error',
        'kebab-case',
        {
          registeredComponentsOnly: false,
          ignores: [],
        },
      ],

      /*
      =====================================
      OPTIONAL STRICTNESS
      =====================================
      */

      // multi word names off
      'vue/multi-word-component-names': 'off',

      // self close flexibility
      'vue/html-self-closing': 'off',

      /*
      =====================================
      COMMON
      =====================================
      */

      'no-console': 'warn',
      'no-debugger': 'error',
    },
  },

  /*
  -------------------------
  PRETTIER LAST
  -------------------------
  */
  prettier,
];
