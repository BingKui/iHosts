/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'vue3-smooth-dnd';
declare module 'vue3-contextmenu';
declare module 'v-contextmenu';
declare module '*.css';
declare var Notification;