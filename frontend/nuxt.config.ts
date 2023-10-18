// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  runtimeConfig: {
    public: {
      wsUrl: process.env.WS_URL
    }
  },
  modules: ["@vueuse/nuxt", "@nuxtjs/tailwindcss"]
})