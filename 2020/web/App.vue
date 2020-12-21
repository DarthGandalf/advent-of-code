<template>
	<v-app>
		<v-navigation-drawer app v-model="menu_visible">
			<v-list nav dense>
				<v-list-item two-line v-for="([num, name, visual], i) in days" :key="'day' + i" :href="'day' + num + '.html'">
					<v-list-item-action class="mr-2"><v-icon v-if="num === current_day" title="Selected day">{{mdiArrowRightBold}}</v-icon></v-list-item-action>
					<v-list-item-content>
						<v-list-item-title class="aoc-em">Day {{num}}</v-list-item-title>
						<v-list-item-subtitle>{{name}}</v-list-item-subtitle>
					</v-list-item-content>
					<v-list-item-action v-if="visual"><v-icon title="Visualization">{{mdiEye}}</v-icon></v-list-item-action>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>
		<v-app-bar app>
			<v-app-bar-nav-icon @click.stop="menu_visible = !menu_visible"></v-app-bar-nav-icon>
			<v-toolbar-title>Day {{current_day}}: <a :href="'https://adventofcode.com/2020/day/' + current_day">{{days[current_day-1][1]}}</a></v-toolbar-title>
		</v-app-bar>
		<v-main>
			<v-skeleton-loader v-show="loading" type="article@3"></v-skeleton-loader>
			<v-container fluid v-show="!loading">
				<v-textarea outlined label="Input" v-model="input"></v-textarea>
				<v-btn outlined :disabled="in_progress" id=run class="ma-3" @click="_run()">Run</v-btn>
				<div class="d-flex">
					<v-textarea outlined v-model="output1" label="Part 1"></v-textarea>
					<v-textarea outlined v-model="output2" label="Part 2"></v-textarea>
				</div>
				<v-card v-show="visual_supported">
					<v-card-title>Visualization</v-card-title>
					<v-card-text>
						<v-checkbox v-model="visual_enabled" label="Enabled"></v-checkbox>
						<v-slider v-model="visual_speed" min=0 max=100 label="Speed"></v-slider>
						<canvas ref="canvas" id="canvas" oncontextmenu="event.preventDefault()"></canvas><br/>
						<v-btn outlined id=stop class=ma-3 @click="_stop()" :disabled="!in_progress">Stop</v-btn>
						<div>
							If you see visualization glitches, try <a href="?renderer=software">this</a> and contact me.
						</div>
					</v-card-text>
				</v-card>
				Source code is <a href="https://github.com/DarthGandalf/advent-of-code/tree/master/2020">over here</a>.
			</v-container>
		</v-main>
	</v-app>
</template>

<script>
import {
	mdiEye,
	mdiArrowRightBold,
} from '@mdi/js';

export default {
	props: {
		days: Array,
		current_day: Number,
	},
	data() {
		return {
			menu_visible: null,
			loading: true,
			input: '',
			output1: '',
			output2: '',
			visual_supported: false,
			visual_enabled: true,
			visual_speed: 90,
			in_progress: false,
			should_stop: false,
			visual_cb: null,
			visual_ptr: null,
			mdiEye,
			mdiArrowRightBold,
		}
	},
	mounted() {
		const ro = new ResizeObserver(entries => {
			for (let entry of entries) {
				const {width, height} = entry.contentRect;
				if (this.visual_cb) {
					this.visual_cb(this.visual_ptr, width, height);
				}
			}
		});
		ro.observe(this.$refs.canvas);
	},
	methods: {
		_run() {
			if (this.visual_supported && this.visual_enabled) {
				this.$vuetify.goTo('#canvas');
			}
		},
		_stop() {
			this.should_stop = true;
		},
		getInput() {
			return this.input;
		},
		setInput(input) {
			this.input = input;
		},
		setOutput1(output) {
			this.output1 = output;
		},
		setOutput2(output) {
			this.output2 = output;
		},
		supportVisual(cb, ptr) {
			this.visual_supported = true;
			this.visual_cb = cb;
			this.visual_ptr = ptr;
		},
		visualEnabled() {
			return this.visual_enabled && !this.should_stop;
		},
		getVisualSpeed() {
			return this.visual_speed;
		},
		setVisualSpeed(speed) {
			this.visual_speed = speed;
		},
		finishVisual() {
			this.$vuetify.goTo(0);
		},
		setInProgress(in_progress) {
			this.in_progress = in_progress;
			if (in_progress) {
				this.should_stop = false;
			}
		},
		setLoaded() {
			this.loading = false;
		}
	}
}
</script>

<style lang="scss">
body {
	background-color: $aoc-background;
	#app {
		.v-btn {
			text-transform: none;
		}
		div {
			.aoc-em {
				color: #ffffff;
				text-shadow: 0 0 5px #ffffff;
			}
		}
		a, button#run {
			text-decoration: none;
			color: #009900;
		}
		a:hover, a:focus, button#run:hover {
			color: #99ff99;
		}
		button#stop {
			text-decoration: none;
			color: #990000;
		}
		button#stop:hover {
			color: #ff0000;
		}
	}
}
</style>
