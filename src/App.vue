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

onMounted(() => {
  appWindow.listen("setup_ok", (v) => {
    console.log(v);
  });
});
</script>

<template>
  <div class="app">
    <h1>抢占式开发工具</h1>
    <div class="options">
      <n-button type="primary" @click="optionHandle(OnOff.open)">
        开启
      </n-button>
      <n-button type="error" @click="optionHandle(OnOff.close)">
        关闭
      </n-button>
    </div>
    <h2>当前状态</h2>
    <n-progress
      type="line"
      :percentage="60"
      :indicator-placement="'inside'"
      processing
    />
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
