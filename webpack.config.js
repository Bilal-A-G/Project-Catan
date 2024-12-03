// Generated using webpack-cli https://github.com/webpack/webpack-cli

const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const util = require('node:util');
const child_process = require('node:child_process');

const exec = util.promisify(child_process.exec);

const isProduction = process.env.NODE_ENV === 'production';
const isClean = process.env.NODE_ENV === 'clean';

const stylesHandler = MiniCssExtractPlugin.loader;

//This plugin simply runs arbitrary npm commands, and logs the output of them with some formatting
//after webpack compiles everything
class WatchModePlugin {
    constructor(commands){
        this.commands = commands;
    }

    apply(compiler) {
      const logger = compiler.getInfrastructureLogger('WatchModePlugin');
      compiler.hooks.done.tap('WatchModePlugin', (stats) => {
        (async () => {
            const horizontalBarLeft = " \n \n ||----------------------------------------||| ";
            const horizontalBarRight = " |||----------------------------------------|| \n";

            let statuses = [];

            for(let i = 0; i < this.commands.length; i++){
                try {
                    statuses[i] = await exec(this.commands[i].command);
                } catch (error){
                    logger.info(horizontalBarLeft + this.commands[i].display + horizontalBarRight);
                    logger.error((error.stdout + error.stderr))
                    statuses[i] = {err: true, mess:  (error.stdout + error.stderr)}
                }
            }

            logger.info(horizontalBarLeft + "OVERALL" + horizontalBarRight);

            for(let i = 0; i < statuses.length; i++){
                if(statuses[i].err){
                    logger.error(this.commands[i].failed);
                }
                else{
                    logger.info(this.commands[i].success);
                }
            }

            logger.warn("Using this plugin may slow down build times, disable it in production!");
        })();
      });
    }
};

const config = {
    entry: './src/app.ts',
    output: {
        path: path.resolve(__dirname + "/public", 'dist'),
        clean: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './public/index.html',
        }),

        new MiniCssExtractPlugin(),

        // Add your plugins here
        // Learn more about plugins from https://webpack.js.org/configuration/plugins/
    ],
    module: {
        rules: [
            {
                test: /\.(ts|tsx)$/i,
                loader: 'ts-loader',
                exclude: ['/node_modules/'],
            },
            {
                test: /\.css$/i,
                use: [stylesHandler,'css-loader'],
            },
            {
                test: /\.s[ac]ss$/i,
                use: [stylesHandler, 'css-loader', 'sass-loader'],
            },
            {
                test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/i,
                type: 'asset',
            },

            // Add your rules for custom modules here
            // Learn more about loaders from https://webpack.js.org/loaders/
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.jsx', '.js', '...'],
    },
};

if(isClean){
    config.plugins.push(new WatchModePlugin([ 
        {display: "TESTS", command: "npm run test", success: "\u2713 tests successful", failed: "\u274c tests failed"},
        {display: "LINTER", command: "npm run lint", success: "\u2713 linter successful", failed: "\u274c linter failed"}
    ]));
}

module.exports = {
    entry: config.entry,
    output: config.output,
    resolve: config.resolve,
    plugins: config.plugins,
    module: config.module,

    mode: isProduction ? 'production' : 'development',
    devtool: isProduction ? false : 'source-map',
    
    devServer: {
        hot: true,
        open: true,
        port: 5000,

        devMiddleware: {
            writeToDisk: true
        }
    }
};
