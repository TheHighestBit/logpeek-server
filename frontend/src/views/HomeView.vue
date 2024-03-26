<template>
  <v-container fluid>
    <v-row>
      <SystemInfo></SystemInfo>
    </v-row>
    <v-row>
      <ApplicationSelect class="ml-3" v-model:application="selected_apps" @update:application="refresh_dashboard"></ApplicationSelect>
    </v-row>
    <v-row>
      <v-col cols="5">
        <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_24" :is_week="false" sparkbar_title="Errors Per Hour" type="error"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_24" :is_week="false" sparkbar_title="Warnings Per Hour" type="warning"></SmallSparkline>
        <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_24" :is_week="false" sparkbar_title="Log Entries Per Hour" type="total"></SmallSparkline>
        <ErrorCountByModule :data="dashboard_info.top_modules_24" card_title="24h errors by module"></ErrorCountByModule>
      </v-col>
      <v-col cols="5">
        <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_week" :is_week="true" sparkbar_title="Errors Per Day" type="error"></SmallSparkline>
        <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_week" :is_week="true" sparkbar_title="Warnings Per Day" type="warning"></SmallSparkline>
        <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_week" :is_week="true" sparkbar_title="Log Entires Per Day" type="total"></SmallSparkline>
        <ErrorCountByModule :data="dashboard_info.top_modules_week" card_title="7d errors by module"></ErrorCountByModule>
      </v-col>
      <v-col>
        <VueUi3dBar :key="barKey" :config="config" :dataset="dataset"></VueUi3dBar>
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
import ApplicationSelect from "@/components/ApplicationSelect.vue";

const store = useAppStore();
const selected_apps = ref<string[]>(
  JSON.parse(sessionStorage.getItem("selected_apps") || "[]")
);

const construct_search_params = () => {
  if (selected_apps.value.length > 0) {
    const search_params = new URLSearchParams();
    selected_apps.value.forEach((app) => search_params.append("applications", app));

    return search_params;
  } else {
    return "";
  }
}

const dashboard_info = ref<DashboardInfo>(await fetchWithAuth("/api/dashboard_info?" + construct_search_params()).then((res) => res.json())
  .catch(() => {
    store.showSnackbar("Failed to fetch dashboard info", "error");
  }));


const barKey = ref(0);
const dataset = ref<VueUi3dBarDataset>({
  percentage: dashboard_info.value.log_buffer_usage,
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
          text: dashboard_info.value.total_log_entries + " in buffer",
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

const refresh_dashboard = async () => {
  if (selected_apps.value.length > 0) {
    dashboard_info.value = await fetchWithAuth("/api/dashboard_info?" + construct_search_params()).then((res) => res.json())
      .catch(() => {
        store.showSnackbar("Failed to fetch dashboard info", "error");
      });
  } else {
    dashboard_info.value = await fetchWithAuth("/api/dashboard_info").then((res) => res.json())
      .catch(() => {
        store.showSnackbar("Failed to fetch dashboard info", "error");
      });
  }

  sessionStorage.setItem("selected_apps", JSON.stringify(selected_apps.value));

  dataset.value = {
    percentage: dashboard_info.value.log_buffer_usage,
  };

  config.value.style!.chart!.title!.subtitle!.text = dashboard_info.value.total_log_entries + " in buffer";
  barKey.value += 1; // Force the component to re-render
}
</script>

