<template>
  <VueUiSparkline
    ref="sparkline"
    :config="config"
    :dataset="dataset"
  ></VueUiSparkline>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { VueUiSparkline } from "vue-data-ui";
import {
  type VueUiSparklineConfig,
  type VueUiSparklineDatasetItem,
} from "vue-data-ui";

const props = defineProps({
  bar_color: String,
  sparkbar_title: String,
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

const dataset = ref<VueUiSparklineDatasetItem[]>([
  { period: "21:37", value: 4 },
  { period: "22:37", value: 77 },
  { period: "23:37", value: 61 },
  { period: "00:37", value: 97 },
  { period: "01:37", value: 47 },
  { period: "02:37", value: 83 },
  { period: "03:37", value: 5 },
  { period: "04:37", value: 57 },
  { period: "05:37", value: 55 },
  { period: "06:37", value: 77 },
  { period: "07:37", value: 71 },
  { period: "08:37", value: 97 },
  { period: "09:37", value: 8 },
  { period: "10:37", value: 100 },
  { period: "11:37", value: 65 },
  { period: "12:37", value: 50 },
  { period: "13:37", value: 36 },
  { period: "14:37", value: 37 },
  { period: "15:37", value: 71 },
  { period: "16:37", value: 28 },
  { period: "17:37", value: 82 },
  { period: "18:37", value: 40 },
  { period: "19:37", value: 67 },
  { period: "20:37", value: 6 }
]);

</script>
