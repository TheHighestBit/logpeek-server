<template>
  <VueUiSparkgauge :config="config" :dataset="dataset"/>
</template>

<script lang="ts" setup>
import {computed, PropType, ref} from "vue";
import {VueUiSparkgauge} from "vue-data-ui";
import "vue-data-ui/style.css";

const props = defineProps({
  bar_color: String,
  gauge_title: String,
  is_week: Boolean,
  data: Array as PropType<number[]>,
  max_value: Number,
});

const config = ref({
  "style": {
    "fontFamily": "inherit",
    "background": "#242424",
    "height": 76,
    "basePosition": 64,
    "animation": {
      "show": true,
      "speedMs": "50"
    },
    "title": {
      "show": true,
      "color": "#ffffff",
      "fontSize": 14,
      "position": "top",
      "textAlign": "center",
      "bold": false
    },
    "dataLabel": {
      "fontSize": 20,
      "autoColor": true,
      "color": "#1A1A1A",
      "offsetY": 0,
      "bold": true,
      "rounding": 2,
      "prefix": "",
      "suffix": ""
    },
    "colors": {
      "min": "#00fd3e",
      "max": props.bar_color,
      "showGradient": true
    },
    "track": {
      "autoColor": true,
      "color": "#5f8bee",
      "strokeLinecap": "round"
    },
    "gutter": {
      "color": "#e1e5e8",
      "strokeLinecap": "round"
    }
  }
});

const dataset = computed(() => {
  if (props.data) {
    return {
      "value": props.data.reduce((a, b) => a + b, 0) / props.data.length,
      "min": 0,
      "max": props.max_value,
      "title": props.gauge_title,
    };
  }

  return {
    "value": 0,
    "min": 0,
    "max": 10,
    "title": props.gauge_title,
  };
})

</script>
