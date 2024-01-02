<template>
  <v-container>
    <v-row>
      <v-col>
        <SmallSparkline :bar_color="colors.primary" sparkbar_title="24h Log Entries"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_error" sparkbar_title="24h Errors"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_warning" sparkbar_title="24h Warnings"></SmallSparkline>
      </v-col>
      <v-col>
        <VueUi3dBar :config="config" :dataset="dataset"></VueUi3dBar>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import SmallSparkline from "@/components/SmallSparkline.vue";
import "vue-data-ui/style.css";
import { colors } from "@/styles/colors";

import {
  type VueUi3dBarConfig,
  type VueUi3dBarDataset,
  VueUi3dBar,
} from "vue-data-ui";

const config = ref<VueUi3dBarConfig>({
  style: {
    fontFamily: "inherit",
    shape: "bar",
    chart: {
      animation: { use: true, speed: 1, acceleration: 1 },
      backgroundColor: "#1A1A1A",
      color: "#CCCCCC",
      bar: { color: "#5f8bee", stroke: "#5f8bee", strokeWidth: 0.7 },
      box: {
        stroke: "#5A5A5A",
        strokeWidth: 0.7,
        strokeDasharray: 2,
        dimensions: {
          width: 128,
          height: 128,
          top: 15,
          bottom: 0,
          left: 24,
          right: 24,
          perspective: 18,
        },
      },
      title: {
        text: "Log buffer used",
        color: "#FAFAFA",
        fontSize: 20,
        bold: true,
      },
      dataLabel: {
        show: true,
        bold: true,
        color: "#5f8bee",
        fontSize: 12,
        rounding: 1,
      },
    },
  },
  userOptions: { show: false },
});

const dataset = ref<VueUi3dBarDataset>({
  percentage: 50,
});
</script>
