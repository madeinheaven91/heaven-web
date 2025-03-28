import { createApp } from 'vue'
import '@/app/style.css'
import App from '@/app/App.vue'
import router from '@/app/router'
import { createPinia } from 'pinia'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
