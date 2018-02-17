import uglify from 'rollup-plugin-uglify'

const production = !process.env.ROLLUP_WATCH

export default {
    input: 'client/main.js',
    output: {
        file: 'static/bundle.js',
        format: 'iife',
        sourcemap: true
    },
    plugins: [
        production && uglify()
    ]
}
