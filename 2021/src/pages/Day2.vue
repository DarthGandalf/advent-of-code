<template>
  <Day :daynum="daynum" :func="solFn"></Day>
</template>

<script lang="ts">
type SolutionFn = (input: string) => number[];

import Day from 'components/Day.vue';
import { defineComponent } from 'vue';
import { solution } from '../../solutions/day2';

const daynum = 2;

export default defineComponent({
  name: 'Day2',
  components: { Day },
  setup() {
    let solFn = solution as SolutionFn;
    return {
      solFn,
      daynum,
    };
  }
});
</script>
