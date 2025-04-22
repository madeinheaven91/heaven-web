import { createApp } from 'vue'
import App from '@/app/App.vue'
import router from '@/app/router'
import { createPinia } from 'pinia'
import '@/app/style.css'
import { config } from 'md-editor-v3';

config({
  editorExtensions: {
    highlight: {
      css: {
        hacker: {
          light: 'https://unpkg.com/highlight.js@11.11.1/styles/hybrid.css',
          dark: 'https://unpkg.com/highlight.js@11.11.1/styles/base16/ashes.css',
        },
      },
    },
  },
});

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
