import path from 'path'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

module.exports = defineConfig({
    build: {
        emptyOutDir: false,
        lib: {
            entry: path.resolve(__dirname, 'src/main.ts'),
            name: 'Plugin',
            fileName: (format) => `plugin.${format}.js`
        }
    },
    plugins: [
        svelte({
            emitCss: false,
        }),
    ]
})
