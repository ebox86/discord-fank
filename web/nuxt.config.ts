// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
      // Target: https://go.nuxtjs.dev/config-target
  target: 'static',

  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: 'Fank: A Discord Bot',
    htmlAttrs: {
      lang: 'en',
    },
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
      { name: 'format-detection', content: 'telephone=no' },

      { hid: 'description', name: 'description', content:  'Fankbot homepage' },
      { hid: 'og:title', property: 'og:title', content: 'Fank: A Discord Bot' },
      { hid: 'og:url', property: 'og:url', content: 'https://fankbot.xyz' },
      { hid: 'og:description', property: 'og:description', content: 'Fankbot homepage' },
      { hid: 'og:image', property: 'og:image', content: "https://fankbot.xyz/_nuxt/logo.1282014f.png"},
      
      // twitter card
      // { hid: "twitter:title", name: "twitter:title", content: this.pageTitle },
      // { hid: "twitter:url", name: "twitter:url", content: this.pageUrl },
      // { hid: 'twitter:description', name: 'twitter:description', content: this.description },
      // { hid: "twitter:image", name: "twitter:image", content: process.env.baseUrl + ogImage},
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
    "@/layouts/global.css",
],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    '@vuestic/nuxt'

  ],
})
