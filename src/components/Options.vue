<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, reactive, ref } from "vue";
import { message } from "ant-design-vue";
enum OnOff {
  open = "open",
  close = "close",
}

const optionHandle = async (type: OnOff) => {
  let res: string;
  switch (type) {
    case OnOff.open:
      res = await invoke("open_ss");
      if (res) {
        state.state = InstanceState.Running;
        state.msg = res;
      }
      break;
    case OnOff.close:
      res = await invoke("close_ss");
      if (res) {
        state.state = InstanceState.Running;
        state.msg = "已关闭";
      }
      break;
  }
  console.log("jiegu", res);

  if (res) {
    message.success("操作成功");
  } else {
    message.error("操作失败");
  }
};
enum InstanceState {
  None,
  Creating,
  Running,
  Closed,
}
const percent = ref(0);

const state = reactive<{ state: InstanceState; msg: string }>({
  state: InstanceState.None,
  msg: "暂无实例",
});

const currentIp = ref("");

onMounted(() => {
  const instance_info = localStorage.getItem("instance_info");
  if (instance_info) {
    const info = JSON.parse(instance_info);
    currentIp.value = info.ip;
    state.msg = info.ip;
  }
});

const createHandle = async () => {
  state.state = InstanceState.Creating;
  state.msg = "创建中...";
  let count = 1;
  const up = () => {
    if (count < 36) {
      count++;
      percent.value = Math.floor((count / 36) * 99);
      setTimeout(up, 1000);
    }
  };
  up();
  const ip: string = await invoke("create_instance");
  percent.value = 100;
  localStorage.setItem("instance_info", JSON.stringify({ ip }));
  state.state = InstanceState.Closed;
  state.msg = ip;
  currentIp.value = ip;
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
    <p>当前ip：{{ currentIp }}</p>
    <p>{{ state.msg }}</p>
    <a-progress
      type="circle"
      v-if="state.state === InstanceState.Creating"
      :percent="percent"
      :width="80"
    />
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
