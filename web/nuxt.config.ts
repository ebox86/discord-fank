// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
      // Target: https://go.nuxtjs.dev/config-target
  target: 'static',

  // Global page headers: https://go.nuxtjs.dev/config-head
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      title: 'Fank: A Discord Bot',
      meta: [
        { 
          name: 'description', 
          content:  'Fankbot homepage' 
        },
        { 
          name: 'og:title', 
          content: 'Fank: A Discord Bot' 
        },
        { 
          name: 'og:url', 
          content: 'https://fankbot.xyz' },
        { 
          property: 'og:description', 
          content: 'Fankbot homepage' 
        },
        { 
          property: 'og:image', 
          content: "https://fankbot.xyz/_nuxt/logo.1282014f.png"
        },
      
      // twitter card
      // { hid: "twitter:title", name: "twitter:title", content: this.pageTitle },
      // { hid: "twitter:url", name: "twitter:url", content: this.pageUrl },
      // { hid: 'twitter:description', name: 'twitter:description', content: this.description },
      // { hid: "twitter:image", name: "twitter:image", content: process.env.baseUrl + ogImage},
    ],
  },
},

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
    "@/assets/css/global.css",
    "@/assets/css/tailwind.css",
],  
  image: {
    // Options
  },

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [],
  publicRuntimeConfig: {
    DISCORD_CLIENT_ID: process.env.DISCORD_CLIENT_ID,
    DISCORD_CLIENT_SECRET: process.env.DISCORD_CLIENT_SECRET,
    REDIRECT_URI: process.env.REDIRECT_URI,
    BOT_URL: process.env.BOT_URL,
  },

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    '@vuestic/nuxt',
    '@nuxt/image-edge',
    '@nuxtjs/device',
    '@nuxtjs/tailwindcss'
  ],
  fontawesome: {
    icons: {
      solid: true,
      brands: true
    }
  },
})
