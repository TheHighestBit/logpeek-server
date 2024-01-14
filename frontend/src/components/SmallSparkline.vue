<template>
  <VueUiSparkline
    ref="sparkline"
    :config="config"
    :dataset="dataset"
  ></VueUiSparkline>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import { PropType } from "vue";
import { ref } from "vue";
import { VueUiSparkline } from "vue-data-ui";
import {
  type VueUiSparklineConfig,
  type VueUiSparklineDatasetItem,
} from "vue-data-ui";

const props = defineProps({
  bar_color: String,
  sparkbar_title: String,
  data: Array as PropType<number[]>,
});

const config = ref<VueUiSparklineConfig>({
  type: "bar",
  style: {
    backgroundColor: "#242424",
    fontFamily: "inherit",
    bar: { borderRadius: 3, color: props.bar_color },
    zeroLine: { color: "#505050", strokeWidth: 1 },
    plot: { show: true, radius: 4, stroke: "#FFFFFF", strokeWidth: 1 },
    verticalIndicator: {
      show: false,
      strokeWidth: 1.5,
      color: "#5f8bee",
      strokeDasharray: 3,
    },
    dataLabel: {
      position: "left",
      fontSize: 48,
      bold: true,
      color: props.bar_color,
      roundingValue: 1,
      valueType: "sum",
    },
    title: {
      show: true,
      textAlign: "left",
      color: "#FAFAFA",
      fontSize: 24,
      bold: true,
      text: props.sparkbar_title,
    },
    area: { show: true, useGradient: true, opacity: 30, color: "#5f8bee" },
  },
});

const dataset = computed(() => {
  if (props.data) {
    const dataset: VueUiSparklineDatasetItem[] = [];
    const currentData = new Date();
    const currentHour = currentData.getHours();
    const currentMinute = currentData.getMinutes();

    for (let i = props.data.length - 1; i >= 0; i--) {
      dataset.push({
        period: `${i + 1} hours ago, ${(currentHour - i - 1 + 24) % 24}:${currentMinute} - ${(currentHour - i + 24) % 24}:${currentMinute} LOCAL`,
        value: props.data[i],
      });
    }
    return dataset;
    }

  return [];
});
</script>
