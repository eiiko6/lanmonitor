<template>
  <div id="page">
    <!-- Settings icon -->
    <button class="settings-icon" @click="handleSettingsClick">
      <i class="fa-solid fa-gear"></i>
    </button>

    <!-- Daemon unreachable config modal -->
    <IpConfig v-if="showIPSettings" :visible="showIPSettings" :ip="ip" :port="port" :errorMessage="errorMessage"
      @update:visible="handleModalClose" @update:daemon="updateDaemon" />

    <!-- Main content -->
    <router-view v-if="errorMessage == ''" v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" :key="$route.path" :daemonIP="daemonIP" :refreshInterval="refreshInterval"
          @open-ip-config="daemonSettings" />
      </transition>
    </router-view>

  </div>
</template>

<script lang="ts">
import { ref, onMounted } from 'vue';
import IpConfig from './components/IpConfig.vue';
import { load, Store } from '@tauri-apps/plugin-store';
import { useRouter, useRoute } from 'vue-router';

export default {
  components: { IpConfig },
  setup() {
    const router = useRouter();
    const route = useRoute();

    const ip = ref('192.168.1.');
    const port = ref(8080);
    const daemonIP = ref('');
    let store: Store;

    const refreshInterval = 1000;

    const errorMessage = ref("");

    const showIPSettings = ref(true);

    const handleSettingsClick = () => {
      if (route.path === '/settings') {
        router.back();
      } else {
        router.push('/settings');
      }
    };

    const daemonUnreachable = () => {
      errorMessage.value = "The daemon is unreachable.";
      showIPSettings.value = true;
    };

    const daemonSettings = () => {
      errorMessage.value = "";
      showIPSettings.value = true;
    };

    const handleModalClose = () => {
      errorMessage.value = "";
      showIPSettings.value = false;
    };

    const updateDaemon = ({ ip: newIP, port: newPort }: { ip: string, port: number }) => {
      ip.value = newIP;
      port.value = newPort;
      daemonIP.value = `http://${newIP}:${newPort}`;
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
        showIPSettings.value = false;
      }

    });

    return { showIPSettings, ip, port, daemonIP, handleModalClose, updateDaemon, errorMessage, daemonUnreachable, daemonSettings, handleSettingsClick, refreshInterval };
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
