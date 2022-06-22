<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { message } from "ant-design-vue";
enum OnOff {
  open = "open",
  close = "close",
}

const optionHandle = async (type: OnOff) => {
  let res: boolean;
  switch (type) {
    case OnOff.open:
      res = await invoke("open_ss");
      break;
    case OnOff.close:
      res = await invoke("close_ss");
      break;
  }
  console.log("jiegu", res);

  if (res) {
    message.success("操作成功");
  } else {
    message.error("操作失败");
  }
};

const percent = ref(0);
const createHandle = async () => {
  let count = 1;
  const up = () => {
    if (count < 36) {
      count++;
      percent.value = Math.floor((count / 36) * 99);
      setTimeout(up, 1000);
    }
  };
  up();
  const res = await invoke("create_instance");
  percent.value = 100;
  console.log("shili结果", res);
};

onMounted(() => {
  appWindow.listen("setup_ok", (v) => {
    console.log(v);
  });

  appWindow.listen("ssr_type", (v) => {
    console.log("实例", v);
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
    <a-progress type="circle" :percent="percent" :width="80" />
  </div>
</template>

<style scoped lang="scss">
.app {
  text-align: center;
  .options {
    display: flex;
    justify-content: space-evenly;
  }
}
</style>
