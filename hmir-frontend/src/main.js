import { createApp } from "vue";
import ElementPlus from 'element-plus'
import router from "./router"
import 'normalize.css/normalize.css'
import 'element-plus/dist/index.css'
import "./style.css";
import App from "./App.vue";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const app = createApp(App)

app.use(ElementPlus).use(router)
app.mount('#app')
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
