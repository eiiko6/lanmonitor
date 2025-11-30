<template>
  <div v-if="visible" class="modal-overlay" @click="overlayClick">
    <div class="modal">
      <h2 v-if="!errorMessage">Daemon Settings</h2>
      <h2 v-else>{{ errorMessage }}</h2>
      <p v-if="!errorMessage">You would get this from the daemon<br>running on your computer.</p>
      <p v-else>Please verify the IP.<br>You would get this from the daemon<br>running on your computer.</p>
      <div class="input-group">
        <label>IP:</label>
        <input v-model="localIP" type="text" />
      </div>
      <div class="input-group">
        <label>Port:</label>
        <input v-model="localPort" type="number" />
      </div>
      <div class="button-group">
        <button @click="saveSettings">Save</button>
        <button @click="close">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { ref, watch, defineComponent, onMounted, onBeforeUnmount } from 'vue';
import { Store, load } from '@tauri-apps/plugin-store';

export default defineComponent({
  props: {
    visible: { type: Boolean, default: false },
    ip: { type: String, required: true },
    port: { type: Number, required: true },
    errorMessage: { type: String, required: false }
  },
  emits: ['update:visible', 'update:daemon'],
  setup(props, { emit }) {
    const localIP = ref(props.ip);
    const localPort = ref(props.port);
    let store: Store;

    watch(() => props.visible, async (val) => {
      if (val) {
        localIP.value = props.ip;
        localPort.value = props.port;
      }
    });

    async function saveSettings() {
      if (!localIP.value || !localPort.value) return;

      emit('update:daemon', { ip: localIP.value, port: localPort.value });
      emit('update:visible', false);

      if (!store) {
        store = await load('store.json', {
          autoSave: false,
          defaults: { ip: props.ip, port: props.port }
        });
      }
      await store.set('ip', { value: localIP.value });
      await store.set('port', { value: localPort.value });
      await store.save();
    }

    function close() {
      emit('update:visible', false);
    }

    function overlayClick(event: MouseEvent) {
      if (event.target === event.currentTarget) {
        close();
      }
    }

    function handleEnter(event: KeyboardEvent) {
      if (event.key === 'Enter') {
        saveSettings();
      }
    }

    onMounted(() => {
      window.addEventListener('keydown', handleEnter);
    });

    onBeforeUnmount(() => {
      window.removeEventListener('keydown', handleEnter);
    });

    return { localIP, localPort, saveSettings, close, overlayClick };
  }
});

</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal {
  background: #43518E33;
  padding: 2rem;
  border-radius: 15px;
  min-width: 300px;
  text-align: center;
  backdrop-filter: blur(50px) brightness(85%) saturate(120%);
  border: 1px solid #dddddd44;
  box-shadow:
    inset 0 0 25px #dddddd20,
    0 4px 30px rgba(0, 0, 0, 0.1);
}

.input-group {
  text-align: left;
  gap: 0.5rem;
}

.button-group {
  margin-top: 1rem;
  display: flex;
  justify-content: space-between;
}

p {
  margin-bottom: 22px;
}
</style>
