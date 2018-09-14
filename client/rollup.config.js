import { terser } from 'rollup-plugin-terser'

const production = !process.env.ROLLUP_WATCH

export default {
    input: 'client/main.js',
    output: {
        file: 'static/bundle.js',
        format: 'iife',
        sourcemap: true
    },
    plugins: [
        production && terser()
    ]
}
