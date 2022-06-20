<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";
import Options from "./components/Options.vue";
import { IConfig } from "./components/Setup.vue";
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
</script>

<template>
  <Setup v-if="!config_storage" @save="saveHandle" />
  <Options v-else />
</template>

<style scoped lang="scss"></style>
