<template>
  <q-page class="row items-center justify-evenly">
    <q-input outlined v-model="input_text" type="textarea"/>
    <q-btn no-caps color="primary" label="Run" @click="run()" />
    <q-input outlined v-model="output_text" type="textarea"/>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'Day',
  props: {
    input: {
      type: String,
      required: true
    },
    func: {
      type: Function,
      required: true,
    },
  },
  setup(props) {
    const input_text = ref(props.input);
    const output_text = ref('');

    function run() {
      output_text.value = props.func(input_text.value) as string;
    }

    return {
      input_text,
      output_text,
      run,
    };
  }
});

</script>
