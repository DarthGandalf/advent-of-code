import 'core-js/stable'
import 'regenerator-runtime/runtime'

import Vue from 'vue'
import Vuetify from 'vuetify/lib'
import App from "./App.vue";
import { days } from '../build-web/alldays.js';

Vue.use(Vuetify);

window.aocvue = new Vue({
	vuetify: new Vuetify({
		theme: {
			dark: true,
			themes: {
				dark: {
					primary: '#ffff66'
				},
			},
		},
		icons: {
			iconfont: 'mdiSvg',
		},
	}),
	el: '#main',
	render: h => h(App, {
		props: {
			days,
			current_day: window.AOCCurrentDay,
		},
	}),
	methods: {
		getInput() {
			return this.$children[0].getInput();
		},
		setInput(input) {
			this.$children[0].setInput(input);
		},
		setOutput1(output) {
			this.$children[0].setOutput1(output);
		},
		setOutput2(output) {
			this.$children[0].setOutput2(output);
		},
		supportVisual() {
			this.$children[0].supportVisual();
		},
		visualEnabled() {
			return this.$children[0].visualEnabled();
		},
		getVisualSpeed() {
			return this.$children[0].getVisualSpeed();
		},
		setVisualSpeed(speed) {
			this.$children[0].setVisualSpeed(speed);
		},
		finishVisual() {
			this.$children[0].finishVisual();
		},
		setInProgress(in_progress) {
			this.$children[0].setInProgress(in_progress);
		},
		setLoaded() {
			this.$children[0].setLoaded();
		},
	},
});
