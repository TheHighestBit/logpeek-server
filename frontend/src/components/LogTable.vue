<template>
  <v-card>
    <v-row>
      <v-col cols="6">
        <VueDatePicker v-model="date_range_filter" range utc time-picker-inline dark></VueDatePicker>
      </v-col>
      <v-col>
        <v-select v-model="min_log_level_filter" clearable label="Min log level" :items="['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR']" density="compact"></v-select>
      </v-col>
      <v-col>
        <v-btn class="mr-5" color="#6716bd" variant="elevated" :onclick="refresh_table">Refresh</v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-text-field v-model="message_filter" label="Message" density="compact"></v-text-field>
      </v-col>
      <v-col>
        <v-text-field v-model="module_filter" label="Module" density="compact"></v-text-field>
      </v-col>
    </v-row>
    <v-data-table-server
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
import {timeArrayToString} from "@/utils";
import {LogTableResponse} from "@/interfaces/LogTableResponse";

const items = ref<LogEntry[]>([]);

const date_range_filter = ref<Date[]>([]);
const min_log_level_filter = ref<string>();
const message_filter = ref<string>();
const module_filter = ref<string>();

const itemsPerPage = ref(10);
const totalItems = ref(0);
const headers = ref([
  { title: "Index", align: "start", key: "index", sortable: false},
  { title: "Timestamp", align: "start", key: "timestamp_formatted", sortable: false },
  { title: "Level", align: "start", key: "level", sortable: false },
  { title: "Module", align: "start", key: "module", sortable: false },
  { title: "Message", align: "start", key: "message", sortable: false }
]);
const loading = ref(true);

const refresh_table = () => {
  load({ page: 1, itemsPerPage: itemsPerPage.value });
};

onMounted(() => {
  window.addEventListener('keydown', enter_pressed);
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', enter_pressed)
});

const enter_pressed = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    refresh_table();
  }
};

const load = async ({ page, itemsPerPage }: { page: number, itemsPerPage: number }) => {
  loading.value = true;
  const search_params = new URLSearchParams({
    page: page.toString(),
    items_per_page: itemsPerPage.toString(),
  });

  if (date_range_filter.value.length > 0) {
    search_params.append("start_timestamp", date_range_filter.value[0].toISOString());
    search_params.append("end_timestamp", date_range_filter.value[1].toISOString());
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

  const response: LogTableResponse = await fetch("/api/log_table?" + search_params).then((res) => res.json());
  response.logs.map((item: LogEntry) => {
    item.index = (page - 1) * itemsPerPage + response.logs.indexOf(item) + 1;
  });

  items.value = response.logs
  totalItems.value = response.total_items;
  loading.value = false;
};

</script>

<style>
  .dp__theme_dark {
    --dp-primary-color: #6716bd;
  }
</style>
