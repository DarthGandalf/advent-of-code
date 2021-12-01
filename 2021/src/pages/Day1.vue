<template>
  <Day :daynum="daynum" :func="solFn"></Day>
</template>

<script lang="ts">
type SolutionFn = (input: string) => string;

import Day from 'components/Day.vue';
import { defineComponent } from 'vue';
import { solution } from '../../solutions/day1';

const daynum = 1;

export default defineComponent({
  name: 'Day1',
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
