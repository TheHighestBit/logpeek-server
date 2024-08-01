<template>
  <VueUiSparkline
    :config="config"
    :dataset="dataset"
    @selectDatapoint="selectDatapoint"
  ></VueUiSparkline>
</template> `

<script lang="ts" setup>
import {computed, PropType, ref} from "vue";
import {VueUiSparkline, type VueUiSparklineConfig, type VueUiSparklineDatasetItem} from "vue-data-ui";
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
    bar: {borderRadius: 3, color: props.bar_color},
    zeroLine: {color: "#505050", strokeWidth: 1},
    plot: {show: true, radius: 4, stroke: "#FFFFFF", strokeWidth: 1},
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
    area: {show: true, useGradient: true, opacity: 30, color: "#5f8bee"},
  },
});

const dataset = computed(() => {
  if (props.data) {
    const dataset = [];
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
        const date = new Date(currentDate);
        date.setUTCDate(currentDate.getUTCDate() - i);
        const displayDate = date.toISOString().split('T')[0];

        dataset.push({
          period: displayDate,
          value: props.data[i],
        });
      }

      const yesterday = new Date(currentDate);
      yesterday.setUTCDate(currentDate.getUTCDate() - 1);
      const displayYesterday = yesterday.toISOString().split('T')[0];

      dataset.push({
        period: "Yesterday, " + displayYesterday,
        value: props.data[1],
      });

      const displayToday = currentDate.toISOString().split('T')[0];

      dataset.push({
        period: "Today, " + displayToday,
        value: props.data[0],
      });

      return dataset;
    }
  }

  return [];
});


function selectDatapoint({index}: { datapoint: VueUiSparklineDatasetItem; index: number; }) {
  let dateStart = new Date();
  let dateEnd = new Date();

  if (props.is_week) {
    dateStart.setDate(dateStart.getDate() - (6 - index));
    dateStart.setHours(0, 0, 0, 0);
    dateEnd.setDate(dateEnd.getDate() - (5 - index));
    dateEnd.setHours(0, 0, 0, 0);

    if (dateEnd > new Date()) {
      dateEnd = new Date();
    }
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
