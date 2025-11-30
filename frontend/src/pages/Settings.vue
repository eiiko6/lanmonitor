<template>
  <div id="content">
    <div class="row">
      <section class="block">
        <h2>Daemon settings</h2>
        <button @click="openIPConfig" class="ip-config-button">
          Configure Daemon IP
        </button>
      </section>

      <section class="block">
        <h2>Connection settings</h2>
      </section>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Settings',
  emits: ['open-ip-config'],
  setup(_, { emit }) {
    const openIPConfig = () => {
      emit('open-ip-config');
    };

    return { openIPConfig };
  },
});
</script>

<style scoped></style>
