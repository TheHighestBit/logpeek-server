<template>
  <div class="d-flex align-stretch">
    <SmallSparkline :bar_color="color" :data="log_data" :is_week="is_week"
                    :type="type" :sparkbar_title="bar_title"></SmallSparkline>
    <spark-gauge :bar_color="color" :gauge_title="gauge_title" :is_week="is_week"
                 :data="log_data" :max_value="upper_limit"
                 class="mb-2 ml-2 pt-1"></spark-gauge>
  </div>
</template>

<script setup lang="ts">
import {colors} from "@/styles/colors";
import SmallSparkline from "@/components/SmallSparkline.vue";
import SparkGauge from "@/components/SparkGauge.vue";
import {computed, PropType} from "vue";

const props = defineProps({
  log_data: Array as PropType<number[]>,
  is_week: Boolean,
  bar_title: String,
  gauge_title: String,
  type: String,
  upper_limit: Number,
});

const color = computed(() => {
  if (props.type === "error") {
    return colors.log_error;
  } else if (props.type === "warning") {
    return colors.log_warning;
  } else {
    return colors.primary;
  }
})
</script>
