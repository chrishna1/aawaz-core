import App from './App.svelte'
import './main.css'

new App({
  target: document.getElementById('root'),
  props: {
    attrs: {
        host: "http://localhost:8080",
        theme: 'light'
    }
  },
})
