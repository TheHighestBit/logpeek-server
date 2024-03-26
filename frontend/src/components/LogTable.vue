<template>
  <ApplicationSelect class="mb-2" v-model:application="selected_apps" @update:application="refresh_table"></ApplicationSelect>
  <v-card height="94vh" border>
    <v-row class="flex-wrap pt-2 mb-n5">
      <v-col sm="5" lg="2" class="ml-2">
        <VueDatePicker v-model="date_range_filter" range utc time-picker-inline dark :preset-dates="presetDates"
        placeholder="Select Timerange" :max-date="new Date()">
          <template #preset-date-range-button="{ label, value, presetDate }">
            <span
              role="button"
              :tabindex="0"
              @click="presetDate(value)"
              @keyup.enter.prevent="presetDate(value)"
              @keyup.space.prevent="presetDate(value)">
              {{ label }}
            </span>
          </template>
        </VueDatePicker>
      </v-col>
      <v-col sm="2" md="2" lg="1">
        <v-select v-model="min_log_level_filter" clearable variant="outlined" label="Min log level"
                  :items="['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR']" density="compact"></v-select>
      </v-col>
      <v-col>
        <v-combobox v-model="message_filter"
                    clearable
                    variant="outlined"
                    label="Message"
                    density="compact"
                    :items="message_history"></v-combobox>
      </v-col>
      <v-col>
        <v-combobox v-model="module_filter"
                    clearable
                    variant="outlined"
                    label="Module"
                    :items="module_history"
                    density="compact"></v-combobox>
      </v-col>
      <v-col class="mr-5" sm="2" lg="1" align="end">
        <v-btn class="mb-n3" color="#6716bd" variant="elevated" :onclick="refresh_table">Refresh</v-btn>
      </v-col>
    </v-row>
    <v-divider class="border-opacity-50"></v-divider>
    <v-data-table-server
      height="calc(97vh - 175px)"
      v-model:items-per-page="itemsPerPage"
      :headers="headers"
      :items-length="totalItems"
      :items="items"
      :loading="loading"
      :items-per-page-options="[10, 25, 100, 500, 1000]"
      @update:options="load"
    >
      <template v-slot:item="i">
        <tr>
          <td>{{ i.item.entry.index }}</td>
          <td>{{ i.item.entry.timestamp }}</td>
          <td v-if="i.item.entry.level === 'ERROR'" style="color: #ff0335">{{ i.item.entry.level }}</td>
          <td v-else-if="i.item.entry.level === 'WARN'" style="color: #FFC107">{{ i.item.entry.level }}</td>
          <td v-else-if="i.item.entry.level === 'INFO'" style="color: #2ebb36">{{ i.item.entry.level }}</td>
          <td v-else-if="i.item.entry.level === 'DEBUG'" style="color: #2196f3">{{ i.item.entry.level }}</td>
          <td v-else style="color: #8764a2">{{ i.item.entry.level }}</td>
          <td>{{ `${i.item.application} -> ${i.item.entry.module}` }}</td>
          <td>{{ i.item.entry.message }}</td>
        </tr>
      </template>
    </v-data-table-server>
  </v-card>
</template>

<script lang="ts" setup>
import {onBeforeUnmount, onMounted, ref} from 'vue'
import VueDatePicker from "@vuepic/vue-datepicker";
import '@vuepic/vue-datepicker/dist/main.css';
import {LogEntryWithApplication} from "@/interfaces/LogEntry";
import {LogTableResponse} from "@/interfaces/LogTableResponse";
import {endOfMonth, startOfMonth, startOfYear, subMonths, subDays} from 'date-fns';
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";
import router from "@/router";
import {useRoute} from "vue-router";
import ApplicationSelect from "@/components/ApplicationSelect.vue";

const items = ref<LogEntryWithApplication[]>([]);
const store = useAppStore();

const date_range_filter = ref<Date[]>([]);
const min_log_level_filter = ref<string>();
const message_filter = ref<string>();
const message_history = ref<string[]>([]);
const module_filter = ref<string>();
const module_history = ref<string[]>([]);
const selected_apps = ref<string[]>(
  JSON.parse(sessionStorage.getItem("selected_apps") || "[]")
);

const loading = ref(true);
const itemsPerPage = ref(10);
const totalItems = ref(0);
const headers = ref([
  {title: "Index", align: "start", key: "index", sortable: false},
  {title: "Timestamp (LOCAL)", align: "start", key: "timestamp_formatted", sortable: false},
  {title: "Level", align: "start", key: "level", sortable: false},
  {title: "Module", align: "start", key: "module", sortable: false},
  {title: "Message", align: "start", key: "message", sortable: false}
]);
const presetDates = ref([
  {label: 'Last 24h', value: [subDays(new Date(), 1), new Date()]},
  {label: 'Last 7 days', value: [subDays(new Date(), 7), new Date()]},
  {label: 'Last 30 days', value: [subDays(new Date(), 30), new Date()]},
  {label: 'This month', value: [startOfMonth(new Date()), new Date()]},
  {
    label: 'Last month',
    value: [startOfMonth(subMonths(new Date(), 1)), endOfMonth(subMonths(new Date(), 1))],
  },
  {label: 'This year', value: [startOfYear(new Date()), new Date()]},
]);

const refresh_table = () => {
  addToHistory();

  load({page: 1, itemsPerPage: itemsPerPage.value});
};

onMounted(() => {
  window.addEventListener('keydown', enter_pressed);

  loadFromHistory();

  // These parameters are set when the user clicks on a particular timeframe in the dashboard
  const route = useRoute();

  if (route.query.dateStart && route.query.dateEnd && route.query.type) {
    date_range_filter.value = [new Date(route.query.dateStart as string), new Date(route.query.dateEnd as string)];

    if (route.query.type === "error") {
      min_log_level_filter.value = "ERROR";
    } else if (route.query.type === "warning") {
      min_log_level_filter.value = "WARN";
    }

    router.replace({query: {}});
    console.log(selected_apps.value)
    refresh_table();
  }
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', enter_pressed);
});

const enter_pressed = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    refresh_table();
  }
};

const load = async ({page, itemsPerPage}: { page: number, itemsPerPage: number }) => {
  loading.value = true;
  const search_params = new URLSearchParams({
    page: page.toString(),
    items_per_page: itemsPerPage.toString(),
  });

  if (date_range_filter.value !== null && date_range_filter.value.length > 0) {
    // For whatever reason the datepicker returns a string instead of a Date object when using user selected date.
    // For the preset dates it returns a Date object, so we need to convert here.
    let start_date = date_range_filter.value[0];
    let end_date = date_range_filter.value[1] !== null ? date_range_filter.value[1] : new Date();

    if (typeof start_date === "string") {
      start_date = new Date(start_date);
    }

    if (typeof end_date === "string") {
      end_date = new Date(end_date);
    }

    search_params.append("start_timestamp", start_date.toISOString());
    search_params.append("end_timestamp", end_date.toISOString());
  }

  if (min_log_level_filter.value) {
    search_params.append("min_log_level", min_log_level_filter.value);
  }

  if (message_filter.value) {
    search_params.append("message", message_filter.value);
  }

  if (module_filter.value) {
    search_params.append("module_name", module_filter.value);
  }

  if (selected_apps.value.length > 0) {
    sessionStorage.setItem("selected_apps", JSON.stringify(selected_apps.value));
    selected_apps.value.forEach((app) => search_params.append("applications", app));
  }

  const response: LogTableResponse = await fetchWithAuth("/api/log_table?" + search_params)
    .then((res) => res.json())
    .catch(() => {
    store.showSnackbar("Error fetching logs", "error");
  });

  response.logs.map((item: LogEntryWithApplication) => {
    item.entry.index = (page - 1) * itemsPerPage + response.logs.indexOf(item) + 1;
    item.entry.timestamp = new Date(item.entry.timestamp).toLocaleString('en-GB');
  });

  items.value = response.logs
  totalItems.value = response.total_items;
  loading.value = false;
};

const addToHistory = () => {
  const max_history_length = 10;
  let needs_save = false;
  if (message_filter.value && !message_history.value.includes(message_filter.value)) {
    if (message_history.value.unshift(message_filter.value) > max_history_length) {
      message_history.value.pop();
    }

    needs_save = true;
  }

  if (module_filter.value && !module_history.value.includes(module_filter.value)) {
    if (module_history.value.unshift(module_filter.value) > max_history_length) {
      module_history.value.pop();
    }

    needs_save = true;
  }

  if (needs_save) {
    saveToHistory();
  }
};

const saveToHistory = () => {
  localStorage.setItem('message_history', JSON.stringify(message_history.value));
  localStorage.setItem('module_history', JSON.stringify(module_history.value));
};

const loadFromHistory = () => {
  const storage_messages = localStorage.getItem('message_history');
  if (storage_messages) {
    message_history.value = JSON.parse(storage_messages);
  }

  const storage_modules = localStorage.getItem('module_history');
  if (storage_modules) {
    module_history.value = JSON.parse(storage_modules);
  }
};
</script>

<style>
.dp__theme_dark {
  --dp-primary-color: #6716bd;
  --dp-border-color: #858585;
  --dp-border-color-hover: #FFFFFF;
}

.v-divider {
  border-color: #6716bd;
}
</style>
