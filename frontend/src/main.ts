import "bootstrap/dist/css/bootstrap.min.css"
import "bootstrap"
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import './app/style.css'
import App from './app/App.vue'
import router from './app/router'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')

