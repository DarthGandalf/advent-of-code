<template>
  <Day :daynum="daynum" :func="solFn"></Day>
</template>

<script lang="ts">
type SolutionFn = (input: string) => number[];

import Day from 'components/Day.vue';
import { defineComponent } from 'vue';
import { solution } from '../../solutions/day6';

const daynum = 6;

export default defineComponent({
  name: 'Day6',
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
