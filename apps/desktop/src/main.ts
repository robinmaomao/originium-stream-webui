import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import { i18n } from './locales'
import { vuetify } from './plugins/vuetify'
import './styles/global.scss'

createApp(App).use(createPinia()).use(router).use(vuetify).use(i18n).mount('#app')
