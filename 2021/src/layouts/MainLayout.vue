<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />

        <q-toolbar-title>
          Advent of Code 2021 Solutions
        </q-toolbar-title>

        <div></div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          Days
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import EssentialLink from 'components/EssentialLink.vue'

const linksList = [{
    title: 'Day 1',
    caption: 'Sonar Sweep',
    link: 'day-1'
  }, {
    title: 'Day 2',
    caption: 'Dive!',
    link: 'day-2'
  }, {
    title: 'Day 3',
    caption: 'Binary Diagnostic',
    link: 'day-3'
  }, {
    title: 'Day 4',
    caption: 'Giant Squid',
    link: 'day-4'
  }, {
    title: 'Day 5',
    caption: 'Hydrothermal Venture',
    link: 'day-5'
  }, {
    title: 'Day 6',
    caption: 'Lanternfish',
    link: 'day-6'
  }, {
    title: 'Day 7',
    caption: 'The Treachery of Whales',
    link: 'day-7'
  }, {
    title: 'Day 8',
    caption: 'Seven Segment Search',
    link: 'day-8'
  }, {
    title: 'Day 9',
    caption: 'Smoke Basin',
    link: 'day-9'
  }, {
    title: 'Day 10',
    caption: 'Syntax Scoring',
    link: 'day-10'
  }, {
    title: 'Day 11',
    caption: 'Dumbo Octopus',
    link: 'day-11'
  }, {
    title: 'Day 12',
    caption: 'Passage Pathing',
    link: 'day-12'
  }, {
    title: 'Day 13',
    caption: 'Transparent Origami',
    link: 'day-13'
  }, {
    title: 'Day 14',
    caption: 'Extended Polymerization',
    link: 'day-14'
  }, {
    title: 'Day 15',
    caption: 'Chiton',
    link: 'day-15'
  }, {
    title: 'Day 16',
    caption: 'Packet Decoder',
    link: 'day-16'
  }, {
    title: 'Day 17',
    caption: 'Trick Shot',
    link: 'day-17'
  }, {
    title: 'Day 18',
    caption: 'Snailfish',
    link: 'day-18'
  }, {
    title: 'Day 19',
    caption: 'Beacon Scanner',
    link: 'day-19'
  }, {
    title: 'Day 20',
    caption: 'Trench Map',
    link: 'day-20'
  }, {
    title: 'Day 21',
    caption: 'Dirac Dice',
    link: 'day-21'
  }, {
    title: 'Day 22',
    caption: 'Reactor Reboot',
    link: 'day-22'
  }];

import { defineComponent, ref } from 'vue'

export default defineComponent({
  name: 'MainLayout',

  components: {
    EssentialLink
  },

  setup () {
    const leftDrawerOpen = ref(false)

    return {
      essentialLinks: linksList,
      leftDrawerOpen,
      toggleLeftDrawer () {
        leftDrawerOpen.value = !leftDrawerOpen.value
      }
    }
  }
})
</script>
