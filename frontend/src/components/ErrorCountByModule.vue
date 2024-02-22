<template>
  <v-card :title="props.card_title" rounded="0" class="pl-3 pr-3 pb-3">
    <VueUiSparkbar
      :config="config"
      :dataset="dataset"
    ></VueUiSparkbar>
  </v-card>
</template>

<script setup lang="ts">
import {VueUiSparkbar, VueUiSparkbarConfig, VueUiSparkbarDatasetItem} from "vue-data-ui";
import {computed, PropType, ref} from "vue";

const props = defineProps({
  card_title: String,
  data: Array as PropType<[string, number][]>,
});

const config = ref<VueUiSparkbarConfig>({
  style: {
    fontFamily: "inherit",
    layout: {independant: true, percentage: true, target: 0},
    gutter: {backgroundColor: "#3A3A3A", opacity: 100},
    bar: {gradient: {show: true, intensity: 40, underlayerColor: "#FFFFFF"}},
    labels: {
      fontSize: 16,
      name: {position: "top", width: "100%", color: "#BABABA", bold: false},
      value: {show: true, bold: true}
    },
    gap: 4
  }
});

const dataset = computed(() => {
  if (props.data) {
    const dataset: VueUiSparkbarDatasetItem[] = [];

    for (const item of props.data) {
      dataset.push({
        name: item[0],
        value: item[1],
        suffix: "%"
      });
    }

    return dataset;
  }

  return [];
});

</script>

<style scoped>

</style>
