import "bootstrap/dist/css/bootstrap.min.css"
import "bootstrap"
import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router'
import store from "./shared/store"


const app = createApp(App)
app.use(store)
app.use(router)
app.mount('#app')

