import Vue from 'vue'
import VueI18n from 'vue-i18n'

Vue.use(VueI18n)

function loadLocaleMessages () {
  const messages = {}

  messages.en = require("./locales/en").default;
  messages.es = require("./locales/es").default;

  return messages
}

export default new VueI18n({
  locale: navigator.language,
  fallbackLocale: "en",
  messages: loadLocaleMessages()
})
