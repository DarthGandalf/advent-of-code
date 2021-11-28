<template>
  <Day :input="input" :func="solFn"></Day>
</template>

<script lang="ts">
import Day from 'components/Day.vue';
import { defineComponent } from 'vue';
import solution from '../../solutions/day1';

type SolutionFn = (input: string) => string;

export default defineComponent({
  name: 'Day1',
  components: { Day },
  setup() {
    let solFn = solution as SolutionFn;
    return {
      input: 'input',
      solFn,
    };
  }
});
</script>
