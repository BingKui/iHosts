import { createApp } from 'vue'
import { createPinia } from 'pinia';
import App from './App.vue'
import contextmenu from 'vue3-contextmenu';
import { ElNotification } from 'element-plus';
import './styles/main.less';
import 'vue3-perfect-scrollbar/dist/vue3-perfect-scrollbar.css';
import 'vue3-contextmenu/dist/vue3-contextmenu.css';

const pinia = createPinia();
const app = createApp(App);

app.use(pinia);
app.use(contextmenu);
app.use(ElNotification);

app.mount('#app')
