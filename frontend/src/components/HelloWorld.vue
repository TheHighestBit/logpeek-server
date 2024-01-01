<template>
  <v-container class="fill-height">
    <v-navigation-drawer
        expand-on-hover
        rail
      >
        <v-list>
          <v-list-item
            prepend-avatar="https://randomuser.me/api/portraits/women/85.jpg"
            title="Sandra Adams"
            subtitle="sandra_a88@gmailcom"
          ></v-list-item>
        </v-list>

        <v-divider></v-divider>

        <v-list density="compact" nav>
          <v-list-item prepend-icon="mdi-folder" title="My Files" value="myfiles"></v-list-item>
          <v-list-item prepend-icon="mdi-account-multiple" title="Shared with me" value="shared"></v-list-item>
          <v-list-item prepend-icon="mdi-star" title="Starred" value="starred"></v-list-item>
        </v-list>
      </v-navigation-drawer>

      <VueUiXy
        ref="xyChart"
        :config="config"
        :dataset="dataset"
        ></VueUiXy>

  </v-container>
</template>

<script lang="ts" setup>
  import { ref } from 'vue'
  import { VueUiXy } from "vue-data-ui";
  import { type VueUiXyConfig, type VueUiXyDatasetItem } from "vue-data-ui";
  import "vue-data-ui/style.css";

  const config = ref<VueUiXyConfig>({"useCssAnimation":true,"useCanvas":false,"chart":{"backgroundColor":"#1A1A1A","color":"#c8c8c8","height":300,"width":500,"highlighter":{"color":"#e1e5e8","opacity":10},"highlightArea":{"show":true,"from":5,"to":8,"color":"#FF0000","opacity":10,"caption":{"text":"This area is highlighted","fontSize":10,"color":"#FAFAFA","bold":false,"offsetY":0,"width":"auto","padding":3,"textAlign":"center"}},"zoom":{"show":true,"color":"#42d392"},"padding":{"top":36,"right":12,"bottom":36,"left":48},"grid":{"stroke":"#e1e5e8","showVerticalLines":false,"labels":{"show":true,"color":"#c8c8c8","fontSize":12,"axis":{"yLabel":"yLabel","xLabel":"xLabel","fontSize":12},"xAxisLabels":{"color":"#c8c8c8","show":true,"showOnlyFirstAndLast":false,"values":["JAN","FEB","MAR","APR","JUN","JUL","AUG","SEP","OCT","NOV","DEC","JAN","FEB","MAR","APR","JUN","JUL","AUG","SEP","OCT","NOV"],"fontSize":6}}},"labels":{"fontSize":7},"legend":{"show":true,"color":"#c8c8c8","useDiv":true,"fontSize":16},"title":{"show":true,"text":"Title","color":"#c8c8c8","fontSize":20,"bold":true,"subtitle":{"fontSize":16,"color":"#c8c8c8","text":"Subtitle"}},"tooltip":{"color":"#FFFFFF","backgroundColor":"#1A1A1A","show":true,"showValue":true,"showPercentage":false,"roundingValue":0,"roundingPercentage":0},"userOptions":{"show":true,"title":"options","labels":{"dataLabels":"Show datalabels","titleInside":"Title inside","legendInside":"Legend inside","showTable":"Show table"}}},"bar":{"borderRadius":2,"useGradient":true,"labels":{"show":true,"offsetY":-6,"rounding":0,"color":"#c8c8c8"}},"line":{"radius":3,"useGradient":true,"strokeWidth":2,"labels":{"show":true,"offsetY":-6,"rounding":0,"color":"#c8c8c8"},"area":{"useGradient":true,"opacity":30}},"plot":{"radius":3,"useGradient":true,"labels":{"show":false,"offsetY":-6,"rounding":0,"color":"#c8c8c8"}},"table":{"rounding":0,"th":{"backgroundColor":"#1A1A1A","color":"#c8c8c8","outline":"1px solid #e1e5e8"},"td":{"backgroundColor":"#1A1A1A","color":"#c8c8c8","outline":"1px solid #e1e5e8"}}});

  const dataset = ref<VueUiXyDatasetItem[]>([
      {
          name: "Series 1",
          series: [-55, -34, -21, -13, -8, -5, -3, -2, -1, -1, 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55],
          type: "bar"
          ,
          color: "rgb(95,139,238)",
      },
      {
          name: "Series 2",
          series: [55, 34, 21, 13, 8, 5, 3, 2, 1, 1, 0, -1, -1, -2, -3, -5, -8, -13, -21, -34, -55],
          type:
              "line"
          ,
          color: "rgb(66,211,146)",
          useProgression: true,
          dataLabels: false
      },
      {
          name: "Series 3",
          series: [64, 60, 52, 42, 30, 16, 0, -18, -38, -46, -50, -46, -38, -18, 0, 16, 30, 42, 52, 60, 64],
          type:
              "plot"
          ,
          color: "rgb(255,100,0)"
      },
      {
          name: "Target",
          series: [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30],
          type: "line",
          color: "#404040",
          dashed: true,
          useTag: "start",
          dataLabels: false,
      },
  ]);
</script>
