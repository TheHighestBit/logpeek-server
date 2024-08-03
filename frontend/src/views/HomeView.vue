<template>
  <v-container fluid>
    <v-row>
      <SystemInfo></SystemInfo>
    </v-row>
    <v-row>
      <ApplicationSelect class="ml-3" v-model:application="selected_app"
                         @update:application="refresh_dashboard"></ApplicationSelect>
    </v-row>
    <div :key="mainkey" class="mt-3">
      <v-row>
        <v-col cols="5">
          <v-row class="mb-n4">
            <v-col>
              <v-banner>
                <template v-slot:text>
                  <p class="text-h5">Last 24h Breakdown</p>
                </template>
              </v-banner>
            </v-col>
          </v-row>
          <v-row class="mt-0">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_24" :is_week="false"
                              type="error" sparkbar_title="Errors"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.log_error" gauge_title="Errors Per Hour" :is_week="false"
                           :data="dashboard_info.error_logs_24" :max_value="1"></spark-gauge>
            </v-col>
          </v-row>
          <v-row class="mt-n6">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_24" :is_week="false"
                              type="warning" sparkbar_title="Warnings"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.log_warning" gauge_title="Warnings Per Hour" :is_week="false"
                           :data="dashboard_info.warning_logs_24" :max_value="5"></spark-gauge>
            </v-col>
          </v-row>
          <v-row class="mt-n6">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_24" :is_week="false"
                              type="total" sparkbar_title="All logs"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.primary" gauge_title="Logs Per Hour" :is_week="false"
                           :data="dashboard_info.total_logs_24" :max_value="50"></spark-gauge>
            </v-col>
          </v-row>
          <ErrorCountByModule :data="dashboard_info.top_modules_24"
                              card_title="24h errors by module"></ErrorCountByModule>
        </v-col>
        <v-col cols="5">
          <v-row class="mb-n4">
            <v-col>
              <v-banner>
                <template v-slot:text>
                  <p class="text-h5">Last 7 Days Breakdown</p>
                </template>
              </v-banner>
            </v-col>
          </v-row>
          <v-row class="mt-0">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.log_error" :data="dashboard_info.error_logs_week" :is_week="true"
                              type="error" sparkbar_title="Errors"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.log_error" gauge_title="Errors Per Day" :is_week="true"
                           :data="dashboard_info.error_logs_week" :max_value="5"></spark-gauge>
            </v-col>
          </v-row>
          <v-row class="mt-n6">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.log_warning" :data="dashboard_info.warning_logs_week" :is_week="true"
                              type="warning" sparkbar_title="Warnings"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.log_warning" gauge_title="Warnings Per Day" :is_week="true"
                           :data="dashboard_info.warning_logs_week" :max_value="25"></spark-gauge>
            </v-col>
          </v-row>
          <v-row class="mt-n6">
            <v-col cols="9">
              <SmallSparkline :bar_color="colors.primary" :data="dashboard_info.total_logs_week" :is_week="true"
                              type="total" sparkbar_title="All logs"></SmallSparkline>
            </v-col>
            <v-col>
              <spark-gauge :bar_color="colors.primary" gauge_title="Logs Per Day" :is_week="true"
                           :data="dashboard_info.total_logs_week" :max_value="1000"></spark-gauge>
            </v-col>
          </v-row>
          <ErrorCountByModule :data="dashboard_info.top_modules_week"
                              card_title="7d errors by module"></ErrorCountByModule>
        </v-col>
        <v-col>
          <VueUi3dBar :config="config" :dataset="dataset"></VueUi3dBar>
        </v-col>
      </v-row>
    </div>
  </v-container>
</template>


<script lang="ts" setup>
import {ref} from "vue";
import SmallSparkline from "@/components/SmallSparkline.vue";
import "vue-data-ui/style.css";
import {colors} from "@/styles/colors";
import {DashboardInfo} from "@/interfaces/DashboardInfo";

import {VueUi3dBar, type VueUi3dBarConfig, type VueUi3dBarDataset,} from "vue-data-ui";
import SystemInfo from "@/components/SystemInfo.vue";
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";
import ErrorCountByModule from "@/components/ErrorCountByModule.vue";
import ApplicationSelect from "@/components/ApplicationSelect.vue";
import SparkGauge from "@/components/SparkGauge.vue";

const store = useAppStore();
const selected_app = ref<string | undefined>(
  sessionStorage.getItem("selected_app") || undefined
);

const construct_search_params = () => {
  if (selected_app.value) {
    const search_params = new URLSearchParams();
    search_params.append("application", selected_app.value);

    return search_params;
  } else {
    return "";
  }
}

const dashboard_info = ref<DashboardInfo>(await fetchWithAuth("/api/dashboard_info?" + construct_search_params()).then((res) => res.json())
  .catch(() => {
    store.showSnackbar("Failed to fetch dashboard info", "error");
  }));


const mainkey = ref(0);
const dataset = ref<VueUi3dBarDataset>({
  percentage: dashboard_info.value.log_buffer_usage,
});

const config = ref<VueUi3dBarConfig>({
  style: {
    fontFamily: "inherit",
    shape: "bar",
    chart: {
      animation: {use: true, speed: 1, acceleration: 1},
      backgroundColor: "#1A1A1A",
      color: "#CCCCCC",
      bar: {color: colors.primary, stroke: "#5f8bee", strokeWidth: 0.7},
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
  userOptions: {show: false},
});

const refresh_dashboard = async () => {
  if (selected_app.value) {
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

  sessionStorage.setItem("selected_app", selected_app.value || "");

  dataset.value = {
    percentage: dashboard_info.value.log_buffer_usage,
  };

  config.value.style!.chart!.title!.subtitle!.text = dashboard_info.value.total_log_entries + " in buffer";

  mainkey.value += 1; // Force re-render
}
</script>

