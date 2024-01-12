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
    <v-infinite-scroll :key="infinite_scroll_key" :items="items" :onLoad="load">
      <template v-for="(item, index) in items" :key="item">
        <div :class="['pa-2', index % 2 === 0 ? 'bg-grey-lighten-2' : '']">
          Item #{{ item }}
        </div>
      </template>
    </v-infinite-scroll>
  </v-card>
</template>

<script lang="ts" setup>
import {onBeforeUnmount, onMounted, ref} from 'vue'
import { LogEntry } from "@/interfaces/LogEntry";
import VueDatePicker from "@vuepic/vue-datepicker";
import '@vuepic/vue-datepicker/dist/main.css';

const items = ref<LogEntry[]>([]);

const date_range_filter = ref<Date[]>([]);
const min_log_level_filter = ref<string>();
const message_filter = ref<string>();
const module_filter = ref<string>();

const infinite_scroll_key = ref(0);

const refresh_table = () => {
  items.value = [];
  infinite_scroll_key.value++; //Forces the component to rerender
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

const load = async ({ side, done }: { side: string, done: Function }) => {
  const search_params = new URLSearchParams({
    index: items.value.length.toString()
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

  const newItems: LogEntry[] = await fetch("/api/log_table?" + search_params).then((res) => res.json());

  items.value = [...items.value, ...newItems];
  console.log(side);

  if (newItems.length < 25) { //25 are loaded at once
    done('empty');
  } else {
    done('ok');
  }
};

</script>

<style>
  .dp__theme_dark {
    --dp-primary-color: #6716bd;
  }
</style>
