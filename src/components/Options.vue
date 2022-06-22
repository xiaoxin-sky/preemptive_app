<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

enum OnOff {
  open = "open",
  close = "close",
}

const optionHandle = (type: OnOff) => {
  appWindow.emit("onOff", type);
};
const createHandle = () => {
  appWindow.emit("createInstance");
};

onMounted(() => {
  appWindow.listen("setup_ok", (v) => {
    console.log(v);
  });
});
</script>

<template>
  <div class="app">
    <div class="options">
      <a-button type="primary" @click="createHandle">创建</a-button>
      <a-button type="primary" @click="optionHandle(OnOff.open)">开启</a-button>
      <a-button type="error" @click="optionHandle(OnOff.close)">关闭</a-button>
    </div>
    <h2>当前状态</h2>
  </div>
</template>

<style scoped lang="scss">
.app {
  text-align: center;
  .options {
    display: flex;
    justify-content: center;
  }
}
</style>
