import path from 'path';
import * as webpack from 'webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';

const distDir = path.join(__dirname, './dist');

const config: webpack.Configuration = {
    mode: 'development',
    entry: './src/NovelGame.fsproj',
    output: {
        path: distDir,
        filename: 'bundle.js',
    },
    module: {
        rules: [
            {
                test: /\.fs(x|proj)?$/,
                loader: 'fable-loader',
            }
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './index.html',
        }),
    ],
    devServer: {
        contentBase: distDir,
        port: 4000,
    },
};

export default config;
