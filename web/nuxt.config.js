import colors from 'vuetify/es5/util/colors'
require('dotenv').config()

// eslint-disable-next-line no-console
console.info(`This is ${process.env.BUILD} Build`)

export default {
  head: {
    titleTemplate: '%s - discord-bot-web',
    title: 'discord-bot-web',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
    script: [{ src: 'https://js.stripe.com/v3/' }],
  },

  css: [],

  plugins: [],

  components: true,

  buildModules: ['@nuxtjs/dotenv', '@nuxt/typescript-build', '@nuxtjs/vuetify'],

  modules: [
    '@nuxt/content',
    '@nuxtjs/axios',
    '@nuxtjs/pwa',
    'nuxt-stripe-module',
  ],

  axios: {
    proxy: true,
  },

  proxy: {
    '/api/': {
      target: process.env.API_ENDPOINT,
      pathRewrite: { '^/api/': '' },
    },
    '/pay/': {
      target: process.env.STRIPE_ENDPOINT,
      pathRewrite: { '^/pay/': '' },
    },
  },

  pwa: {
    manifest: {
      lang: 'en',
    },
  },

  stripe: {
    publishableKey: process.env.STRIPE_PUBLISHABLE_KEY,
    apiVersion: '2020-08-27',
  },

  vuetify: {
    customVariables: ['~/assets/variables.scss'],
    theme: {
      dark: true,
      themes: {
        dark: {
          primary: colors.blue.darken2,
          accent: colors.grey.darken3,
          secondary: colors.amber.darken3,
          info: colors.teal.lighten1,
          warning: colors.amber.base,
          error: colors.deepOrange.accent4,
          success: colors.green.accent3,
        },
      },
    },
  },

  build: {},
}
