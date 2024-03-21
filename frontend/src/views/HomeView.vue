<template>
  <v-container fluid>
    <v-row>
      <SystemInfo></SystemInfo>
    </v-row>
    <v-row>
      <v-btn-toggle
        v-model="application"
        background-color="primary"
        multiple
      >
        <v-btn
          v-for="app in application_list"
          :key="app"
          :value="app"
        >
          {{ app }}
        </v-btn>
      </v-btn-toggle>
      <v-btn class="mb-3" :color="colors.primary" @click="refresh">
        Refresh
      </v-btn>
    </v-row>
    <v-row>
      <v-col cols="5">
        <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_24" :is_week="false" sparkbar_title="24h Error count" type="error"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_24" :is_week="false" sparkbar_title="24h Warning count" type="warning"></SmallSparkline>
        <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_24" :is_week="false" sparkbar_title="24h Total Log count" type="total"></SmallSparkline>
        <ErrorCountByModule :data="dashboard_info.top_modules_24" card_title="24h errors by module"></ErrorCountByModule>
      </v-col>
      <v-col cols="5">
        <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_week" :is_week="true" sparkbar_title="7d Error count" type="error"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_week" :is_week="true" sparkbar_title="7d Warning count" type="warning"></SmallSparkline>
        <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_week" :is_week="true" sparkbar_title="7d Total Log count" type="total"></SmallSparkline>
        <ErrorCountByModule :data="dashboard_info.top_modules_week" card_title="7d errors by module"></ErrorCountByModule>
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
import { DashboardInfo } from "@/interfaces/DashboardInfo";

import {
  type VueUi3dBarConfig,
  type VueUi3dBarDataset,
  VueUi3dBar,
} from "vue-data-ui";
import SystemInfo from "@/components/SystemInfo.vue";
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";
import ErrorCountByModule from "@/components/ErrorCountByModule.vue";

const store = useAppStore();
const dashboard_info: DashboardInfo = await fetchWithAuth("/api/dashboard_info").then((res) => res.json())
  .catch(() => {
    store.showSnackbar("Failed to fetch dashboard info", "error");
  });

const application = ref<string[]>();
const application_list: string[] = await fetchWithAuth("/api/application_list").then((res) => res.json())
  .catch(() => {
    store.showSnackbar("Failed to fetch applications", "error");
  });

const dataset = ref<VueUi3dBarDataset>({
  percentage: dashboard_info.log_buffer_usage,
});

const config = ref<VueUi3dBarConfig>({
  style: {
    fontFamily: "inherit",
    shape: "bar",
    chart: {
      animation: { use: true, speed: 1, acceleration: 1 },
      backgroundColor: "#1A1A1A",
      color: "#CCCCCC",
      bar: { color: colors.primary, stroke: "#5f8bee", strokeWidth: 0.7 },
      box: {
        stroke: "#5A5A5A",
        strokeWidth: 0.7,
        strokeDasharray: 2,
        dimensions: {
          width: 128,
          height: 128,
          top: 25,
          bottom: 0,
          left: 24,
          right: 24,
          perspective: 8,
        },
      },
      title: {
        text: "Log buffer usage",
        color: "#FAFAFA",
        fontSize: 20,
        bold: true,
        subtitle: {
          text: dashboard_info.total_log_entries + " in buffer",
        }
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

const refresh = () => {
  console.log(application.value);
}
</script>

