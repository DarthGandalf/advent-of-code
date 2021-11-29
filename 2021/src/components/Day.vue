<template>
  <q-page class="row items-center justify-evenly">
    <template v-if="loaded">
      <q-input outlined v-model="input_text" type="textarea"/>
      <q-btn no-caps color="primary" label="Run" @click="run()" />
      <q-input outlined v-model="output_text" type="textarea"/>
    </template>
    <q-skeleton v-else height="200px" width="200px"/>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeMount } from 'vue';

export default defineComponent({
  name: 'Day',
  props: {
    daynum: {
      type: Number,
      required: true
    },
    func: {
      type: Function,
      required: true,
    },
  },
  setup(props) {
    const loaded = ref(false);
    const input_text = ref('');
    const output_text = ref('');

    onBeforeMount(async () => {
      const res = await fetch(`/input/${props.daynum}.txt`);
      const data = await res.text();
      input_text.value = data;
      loaded.value = true;
    });

    function run() {
      output_text.value = props.func(input_text.value) as string;
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
