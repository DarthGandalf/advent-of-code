<template>
  <GenericDay :daynum="dayNum" :key="dayNum"></GenericDay>
</template>

<script lang="ts">
import GenericDay from 'components/GenericDay.vue';
import { defineComponent, ref, watch } from 'vue';
import { useRoute } from 'vue-router'

export default defineComponent({
  name: 'GenericDayPage',
  components: { GenericDay },
  setup() {
    const route = useRoute();
    const daynum = route.params.daynum;
    const dayNum = ref(Number(daynum));

    watch(
      () => route.params.daynum,
      newNum => {
        dayNum.value = Number(newNum);
      },
    );
    return {
      dayNum,
    };
  }
});
</script>
