<template>
  <VueUiSparkline
    :config="config"
    :dataset="dataset"
    @selectDatapoint="selectDatapoint"
  ></VueUiSparkline>
</template> `

<script lang="ts" setup>
import { computed } from "vue";
import { PropType } from "vue";
import { ref } from "vue";
import { VueUiSparkline } from "vue-data-ui";
import {
  type VueUiSparklineConfig,
  type VueUiSparklineDatasetItem,
} from "vue-data-ui";
import router from "@/router";

const props = defineProps({
  bar_color: String,
  sparkbar_title: String,
  is_week: Boolean,
  type: String,
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
    const currentDate = new Date();

    if (!props.is_week) {
      const currentHour = currentDate.getHours();
      const currentMinute = currentDate.getMinutes().toString().padStart(2, "0");

      for (let i = props.data.length - 1; i >= 0; i--) {
        const displayHourStart = ((currentHour - i + 23) % 24).toString().padStart(2, "0");
        const displayHourEnd = ((currentHour - i + 24) % 24).toString().padStart(2, "0");

        dataset.push({
          period: `${i + 1} hours ago, ${displayHourStart}:${currentMinute} - ${displayHourEnd}:${currentMinute} LOCAL`,
          value: props.data[i],
        });
      }

      return dataset;
    } else {
      for (let i = props.data.length - 1; i >= 2; i--) {
        dataset.push({
          period: `${i} days ago`,
          value: props.data[i],
        });
      }

      dataset.push({
        period: "Yesterday",
        value: props.data[1],
      });

      dataset.push({
        period: "Today",
        value: props.data[0],
      })

      return dataset;
    }
  }

  return [];
});

function selectDatapoint({ index }: {datapoint: VueUiSparklineDatasetItem; index: number;}) {
  let dateStart = new Date();
  let dateEnd = new Date();

  if (props.is_week) {
    dateStart.setDate(dateStart.getDate() - (7 - index));
    dateEnd.setDate(dateEnd.getDate() - (6 - index));
  } else {
    dateStart.setHours(dateStart.getHours() - (24 - index));
    dateEnd.setHours(dateEnd.getHours() - (23 - index));
  }

  router.push({
    name: "LogTable",
    query: {
      dateStart: dateStart.toISOString(),
      dateEnd: dateEnd.toISOString(),
      type: props.type,
    },
  });
}
</script>
