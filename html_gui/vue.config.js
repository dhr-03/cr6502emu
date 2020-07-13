//https://github.com/vuejs/vue-cli/issues/2231
module.exports = {
    chainWebpack: config => {
        config.plugin('copy').tap(([options]) => {
            options[0].ignore.push('wasm/*_dev*')
            return [options]
        })
    },

    productionSourceMap: false
}
