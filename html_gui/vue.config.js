//https://github.com/vuejs/vue-cli/issues/2231
module.exports = {
    chainWebpack: config => {
        config.resolve.set('symlinks', false);

        config.plugin('copy').tap(([options]) => {
            options[0].ignore.push('wasm/*_dev*')
            return [options]
        })
    },

    productionSourceMap: false,

    //publicPath: "/cr6502emu_demo/" //used for gh pages
}
