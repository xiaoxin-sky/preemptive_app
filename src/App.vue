<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import Options from "./components/Options.vue";
import { IConfig } from "./components/Setup.vue";
import Drawer from "./components/Drawer.vue";
import { listen } from "@tauri-apps/api/event";
interface Config {
  access_key_id: string;
  access_key_secret: string;
  release_time: string;
  password: string;
}

const config_storage = ref(localStorage.getItem("config"));

const saveHandle = (val: IConfig) => {
  let newVal = JSON.stringify(val);
  localStorage.setItem("config", newVal);
  config_storage.value = newVal;
  appWindow.emit("saveConfig", val);
};
const val = ref();
onMounted(() => {
  listen("message", (e) => {
    val.value = JSON.stringify(e);
  });
});
</script>

<template>
  <div class="app">
    <Drawer @save="saveHandle" />
    <Setup v-if="!config_storage" @save="saveHandle" />
    <Options v-else />
    <div>{{ val }}</div>
  </div>
</template>

<style scoped lang="scss">
.app {
  padding: 12px 16px;
}
</style>
