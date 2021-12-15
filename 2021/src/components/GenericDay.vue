<template>
  <q-page class="row items-center justify-evenly">
    <template v-if="loaded">
      <q-input outlined v-model="input_text" type="textarea"/>
      <q-btn no-caps color="primary" label="Run" @click="run()" />
      <q-input outlined v-model="output_text" type="textarea"/>
    </template>
    <q-spinner v-else color="primary" size="3em"/>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeMount } from 'vue';

type SolutionFile = {
  solution: (input: string) => string[];
}

export default defineComponent({
  name: 'GenericDay',
  props: {
    daynum: {
      type: Number,
      required: true
    },
  },
  setup(props) {
    const loaded = ref(false);
    const input_text = ref('');
    const output_text = ref('');
    let func = (input: string) => [input];

    onBeforeMount(async () => {
      const res = await fetch(`/input/day${props.daynum}.txt`);
      const data = await res.text();
      const solutionFile = await import(`../../solutions/day${props.daynum}`) as SolutionFile;
      const {solution} = solutionFile;
      func = solution;
      input_text.value = data;
      loaded.value = true;
    });

    function run() {
      const results = func(input_text.value);
      output_text.value = `Part 1:\n${results[0]}\n\nPart 2:\n${results[1]}`;
    }

    return {
      input_text,
      output_text,
      run,
      loaded,
    };
  }
});

</script>
