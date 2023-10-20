// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@vueuse/nuxt", "@nuxtjs/tailwindcss"],
  css: ["~/assets/main.css"],
  ssr: false,
  runtimeConfig: {
    public: {
      wsUrl: process.env.WS_URL
    }
  },
})