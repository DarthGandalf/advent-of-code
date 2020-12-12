import 'core-js/stable'
import 'regenerator-runtime/runtime'

import Vue from 'vue'
import Vuetify from 'vuetify/lib'
import App from "./App.vue";

Vue.use(Vuetify);

window.aocvue = new Vue({
	vuetify: new Vuetify({
		theme: {
			dark: true,
		},
		icons: {
			iconfont: 'mdiSvg',
		},
	}),
	el: '#main',
	render: h => h(App, {
		props: {
			days: window.AOCDays,
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
		visualSpeed() {
			return this.$children[0].visualSpeed();
		},
		setInProgress(in_progress) {
			this.$children[0].setInProgress(in_progress);
		},
		setLoaded() {
			this.$children[0].setLoaded();
		},
	},
});
