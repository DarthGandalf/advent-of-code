const webpack = require('webpack');
const VueLoaderPlugin = require('vue-loader/lib/plugin');
const VuetifyLoaderPlugin = require('vuetify-loader/lib/plugin')
const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");
const {CleanWebpackPlugin} = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const postcssPresetEnv = require('postcss-preset-env');
const cssnano = require('cssnano');

const debug = process.env.npm_lifecycle_event !== 'build';

const css_loader = [{
	loader: debug ? 'style-loader' : MiniCssExtractPlugin.loader,
}, {
	loader: 'css-loader',
	options: {
		sourceMap: true,
	}
}, {
	loader: 'postcss-loader',
	options: {
		sourceMap: true,
		ident: 'postcss',
		plugins: [
			postcssPresetEnv(),
		].concat(debug ? [
		] : [
			cssnano({
				preset: 'default',
			}),
		]),
	}
}];

module.exports = {
	mode: debug ? 'development' : 'production',
	entry: './web/main.js',
	output: {
		filename: '[name].js',
		path: path.resolve(__dirname, 'dist'),
	},
	module: {
		rules: [{
			test: /\.vue$/,
			use: 'vue-loader',
		}, {
			test: /\.s?css$/,
			use: css_loader.concat([{
				loader: 'sass-loader',
				options: {
					sourceMap: true,
					additionalData: "@import 'web/variables.scss';",
					sassOptions: {
						includePaths: __dirname,
					},
				}
			}])
		}, {
			test: /\.sass$/,
			use: css_loader.concat([{
				loader: 'sass-loader',
				options: {
					sourceMap: true,
					additionalData: "@import 'web/variables.scss'",
					sassOptions: {
						includePaths: __dirname,
					},
				}
			}])
		}]
	},
	plugins: [
		new webpack.ProgressPlugin(),
		new VueLoaderPlugin(),
		new VuetifyLoaderPlugin(),
		new MiniCssExtractPlugin(),
		new CopyPlugin({
			patterns: [
				{ from: "build-web/day*", toType: "file", to({ absoluteFilename }) {return path.basename(absoluteFilename);}},
			],
		}),
	].concat(debug ? [] : [new CleanWebpackPlugin()]),
	devtool: debug ? 'inline-source-map' : 'source-map',
	optimization: {
		splitChunks: {
			chunks: 'all',
		},
	},
}
