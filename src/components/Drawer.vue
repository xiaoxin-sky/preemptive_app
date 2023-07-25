<template>
  <div class="drawer-setting">
    <SettingOutlined @click="showDrawer" />
    <a-drawer
      v-model:visible="visible"
      class="custom-class"
      style="color: red"
      title="Basic Drawer"
      placement="right"
      @after-visible-change="afterVisibleChange"
    >
      <SetupVue v-bind="$attrs" />
    </a-drawer>
  </div>
</template>
<script lang="ts" setup>
import SetupVue from "./Setup.vue";
import { SettingOutlined } from "@ant-design/icons-vue";
import { ref } from "vue";
const visible = ref<boolean>(false);

const afterVisibleChange = (bool: boolean) => {
  console.log("visible", bool);
};

const showDrawer = () => {
  visible.value = true;
};
</script>
<style lang="scss" scoped>
.drawer-setting {
}
</style>
