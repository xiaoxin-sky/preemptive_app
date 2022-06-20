<template>
  <div class="form-warp">
    <a-form
      :model="modelRef"
      name="basic"
      :label-col="{ span: 8 }"
      :wrapper-col="{ span: 16 }"
      autocomplete="off"
      @finish="onFinish"
      @finishFailed="onFinishFailed"
    >
      <a-form-item
        label="access_key_id"
        name="access_key_id"
        :rules="[{ required: true }]"
      >
        <a-input v-model:value="modelRef.access_key_id" />
      </a-form-item>
      <a-form-item
        label="access_key_secret"
        name="access_key_secret"
        :rules="[{ required: true }]"
      >
        <a-input v-model:value="modelRef.access_key_secret" />
      </a-form-item>
      <a-form-item label="密码" name="password" :rules="[{ required: true }]">
        <a-input v-model:value="modelRef.password" />
      </a-form-item>
      <a-form-item
        label="释放时间"
        name="release_time"
        :rules="[{ required: true }]"
      >
        <a-input v-model:value="modelRef.release_time" />
      </a-form-item>

      <a-form-item :wrapper-col="{ offset: 8, span: 16 }">
        <a-button type="primary" html-type="submit">保存</a-button>
      </a-form-item>
    </a-form>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

export interface IConfig {
  access_key_id: string;
  access_key_secret: string;
  release_time: string;
  password: string;
}

const modelRef = ref<IConfig>({
  access_key_id: "",
  access_key_secret: "",
  release_time: "",
  password: "",
});

const emit = defineEmits<{ (e: "save", val: IConfig): void }>();

const onFinish = () => {
  emit("save", modelRef.value);
};
const onFinishFailed = () => {};
</script>

<style scoped>
.form-warp {
  padding: 16px;
}
</style>
