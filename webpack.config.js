// Generated using webpack-cli https://github.com/webpack/webpack-cli

const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const isProduction = process.env.NODE_ENV == 'production';
const stylesHandler = MiniCssExtractPlugin.loader;

const config = {
    entry: './public/src/app.ts',
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

module.exports = {
    entry: config.entry,
    output: config.output,
    resolve: config.resolve,
    plugins: config.plugins,
    module: config.module,

    mode: isProduction ? 'production' : 'development',
    devtool: false,
    
    devServer: {
        hot: true,
        open: true,
        port: 5000,

        devMiddleware: {
            writeToDisk: true
        }
    }
};
