import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
//import './styles/main.less'
import App from './App.vue'

const app = createApp(App)

app.use(ElementPlus).mount('#app')
