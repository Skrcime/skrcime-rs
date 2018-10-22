import riot  from 'rollup-plugin-riot'
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
        production && terser()
    ],
    external: [ 'riot' ]
}