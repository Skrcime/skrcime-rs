import riot  from 'rollup-plugin-riot'
import buble from 'rollup-plugin-buble'
import { terser } from 'rollup-plugin-terser'

const production = !process.env.ROLLUP_WATCH
const options = {
    ext: 'html'
}

export default {
    input: 'client/main.js',
    output: {
        file: 'static/bundle.js',
        format: 'iife',
        sourcemap: true,
        globals: { riot: 'riot' }
    },
    plugins: [
        riot(options),
        buble(),
        production && terser()
    ],
    external: [ 'riot' ]
}