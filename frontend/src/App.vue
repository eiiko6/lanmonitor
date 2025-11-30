<template>
  <div id="page">
    <!-- Settings icon -->
    <button class="settings-icon" @click="showSettings = true">
      <i class="fa-solid fa-gear"></i>
    </button>

    <!-- IP Modal (initial) -->
    <IPConfig v-if="showModal || showSettings" :visible="showModal || showSettings" :ip="ip" :port="port"
      :errorMessage="errorMessage" @update:visible="handleModalClose" @update:daemon="updateDaemon" />

    <!-- Main content -->
    <router-view v-if="!showModal" v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" :key="$route.path" :daemonIP="daemonIP" />
      </transition>
    </router-view>
  </div>
</template>

<script lang="ts">
import { ref, onMounted } from 'vue';
import IPConfig from './components/IpConfig.vue';
import { load, Store } from '@tauri-apps/plugin-store';

export default {
  components: { IPConfig },
  setup() {
    const showModal = ref(true);
    const showSettings = ref(false);
    const ip = ref('192.168.1.');
    const port = ref(8080);
    const daemonIP = ref('');
    let store: Store;

    const errorMessage = ref("");
    const daemonUnreachable = () => {
      errorMessage.value = "The daemon is unreachable.";
      showModal.value = true;
      showSettings.value = false;
    };

    const handleModalClose = () => {
      showModal.value = false;
      showSettings.value = false;
    };

    const updateDaemon = ({ ip: newIP, port: newPort }: { ip: string, port: number }) => {
      ip.value = newIP;
      port.value = newPort;
      daemonIP.value = `http://${newIP}:${newPort}`;
      // reload Home.vue if needed by forcing key update
      // this works if Home.vue uses :key="$route.path + daemonIP"
    };

    onMounted(async () => {
      window.addEventListener("daemon-unreachable", () => {
        daemonUnreachable();
      });

      store = await load('store.json', { autoSave: false, defaults: {} });

      const savedIP = await store.get<{ value: string }>('ip');
      const savedPort = await store.get<{ value: number }>('port');

      if (savedIP?.value && savedPort?.value) {
        ip.value = savedIP.value;
        port.value = savedPort.value;
        daemonIP.value = `http://${savedIP.value}:${savedPort.value}`;
        showModal.value = false;
      }

    });

    return { showModal, showSettings, ip, port, daemonIP, handleModalClose, updateDaemon, errorMessage, daemonUnreachable };
  },
};
</script>

<style scoped>
#page {
  margin-top: 30px;
  padding-bottom: 30px;
}

.settings-icon {
  position: fixed;
  top: 12px;
  right: 12px;
  font-size: 1.5rem;
  cursor: pointer;
  z-index: 15;
  color: #fff;
}

@media screen and (max-width: 720px) {
  #page {
    margin-top: 30px;
    padding-bottom: 30px;
  }

  .settings-icon {
    top: 42px;
  }
}

@media screen and (max-height: 720px) {
  .settings-icon {
    top: 30px;
    right: 58px;
  }
}
</style>
