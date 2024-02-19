<template>
  <v-card>
    <v-row>
      <v-col cols="7">
        <VueDatePicker v-model="date_range_filter" range utc time-picker-inline dark :preset-dates="presetDates">
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
      <v-col cols="3">
        <v-select v-model="min_log_level_filter" clearable variant="outlined" label="Min log level"
                  :items="['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR']" density="compact"></v-select>
      </v-col>
      <v-col cols="2">
        <v-btn class="mr-5" color="#6716bd" variant="elevated" :onclick="refresh_table">Refresh</v-btn>
      </v-col>
    </v-row>
    <v-row>
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
    </v-row>
    <v-data-table-server
      height="80vh"
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
          <td>{{ i.item.index }}</td>
          <td>{{ i.item.timestamp }}</td>
          <td v-if="i.item.level === 'ERROR'" style="color: #ff0335">{{ i.item.level }}</td>
          <td v-else-if="i.item.level === 'WARN'" style="color: #FFC107">{{ i.item.level }}</td>
          <td v-else-if="i.item.level === 'INFO'" style="color: #2ebb36">{{ i.item.level }}</td>
          <td v-else-if="i.item.level === 'DEBUG'" style="color: #2196f3">{{ i.item.level }}</td>
          <td v-else style="color: #8764a2">{{ i.item.level }}</td>
          <td>{{ i.item.module }}</td>
          <td>{{ i.item.message }}</td>
        </tr>
      </template>
    </v-data-table-server>
  </v-card>
</template>

<script lang="ts" setup>
import {onBeforeUnmount, onMounted, ref} from 'vue'
import VueDatePicker from "@vuepic/vue-datepicker";
import '@vuepic/vue-datepicker/dist/main.css';
import {LogEntry} from "@/interfaces/LogEntry";
import {LogTableResponse} from "@/interfaces/LogTableResponse";
import {endOfMonth, endOfYear, startOfMonth, startOfYear, subMonths, subDays} from 'date-fns';
import {fetchWithAuth} from "@/utils";
import {useAppStore} from "@/store/app";

const items = ref<LogEntry[]>([]);
const store = useAppStore();

const date_range_filter = ref<Date[]>([]);
const min_log_level_filter = ref<string>();
const message_filter = ref<string>();
const message_history = ref<string[]>([]);
const module_filter = ref<string>();
const module_history = ref<string[]>([]);

const loading = ref(true);
const itemsPerPage = ref(10);
const totalItems = ref(0);
const headers = ref([
  {title: "Index", align: "start", key: "index", sortable: false},
  {title: "Timestamp", align: "start", key: "timestamp_formatted", sortable: false},
  {title: "Level", align: "start", key: "level", sortable: false},
  {title: "Module", align: "start", key: "module", sortable: false},
  {title: "Message", align: "start", key: "message", sortable: false}
]);
const presetDates = ref([
  {label: 'Last 24h', value: [subDays(new Date(), 1), new Date()]},
  {label: 'Last 7 days', value: [subDays(new Date(), 7), new Date()]},
  {label: 'Last 30 days', value: [subDays(new Date(), 30), new Date()]},
  {label: 'This month', value: [startOfMonth(new Date()), endOfMonth(new Date())]},
  {
    label: 'Last month',
    value: [startOfMonth(subMonths(new Date(), 1)), endOfMonth(subMonths(new Date(), 1))],
  },
  {label: 'This year', value: [startOfYear(new Date()), endOfYear(new Date())]},
]);

const refresh_table = () => {
  addToHistory();

  load({page: 1, itemsPerPage: itemsPerPage.value});
};

onMounted(() => {
  window.addEventListener('keydown', enter_pressed);

  loadFromHistory();
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
    //TODO show error if this is null
    search_params.append("start_timestamp", date_range_filter.value[0].toString());

    if (date_range_filter.value[1] !== null) {
      search_params.append("end_timestamp", date_range_filter.value[1].toString());
    } else {
      search_params.append("end_timestamp", new Date().toISOString());
    }
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

  const response: LogTableResponse = await fetchWithAuth("/api/log_table?" + search_params)
    .then((res) => res.json())
    .catch(() => {
    store.showSnackbar("Error fetching logs", "error");
  });

  response.logs.map((item: LogEntry) => {
    item.index = (page - 1) * itemsPerPage + response.logs.indexOf(item) + 1;
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
}
</style>
