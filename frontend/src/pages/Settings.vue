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
        <button @click="openRefreshIntervalConfig" class="refresh-interval-button">
          Configure Refresh Interval
        </button>
      </section>
    </div>

    <!-- Refresh Interval Modal -->
    <RefreshIntervalConfig v-if="showRefreshInterval" :visible="showRefreshInterval" :interval="refreshInterval"
      @update:visible="handleRefreshModalClose" @update:interval="updateRefreshInterval" />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import RefreshIntervalConfig from '../components/RefreshIntervalConfig.vue';

export default defineComponent({
  name: 'Settings',
  components: { RefreshIntervalConfig },
  emits: ['open-ip-config', 'update:refresh-interval'],
  props: {
    refreshInterval: { type: Number, required: true }
  },
  setup(_, { emit }) {
    const showRefreshInterval = ref(false);

    const openRefreshIntervalConfig = () => {
      showRefreshInterval.value = true;
    };

    const handleRefreshModalClose = () => {
      showRefreshInterval.value = false;
    };

    const updateRefreshInterval = (interval: number) => {
      emit('update:refresh-interval', interval);
    };

    const openIPConfig = () => {
      emit('open-ip-config');
    };

    return { showRefreshInterval, openRefreshIntervalConfig, handleRefreshModalClose, updateRefreshInterval, openIPConfig };
  }
});
</script>
