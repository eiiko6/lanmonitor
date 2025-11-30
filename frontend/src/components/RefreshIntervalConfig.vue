<template>
  <GenericPopup :visible="showError" title="Invalid Interval"
    message="Refresh interval cannot be below 500ms. Automatically set to 500ms."
    @update:visible="showError = $event" />

  <div v-if="visible" class="modal-overlay" @click="overlayClick">
    <div class="modal">
      <h2>Refresh Interval</h2>
      <p>Set how frequently the app fetches data from the daemon (in ms).</p>
      <div class="input-group">
        <label>Interval:</label>
        <input type="number" v-model.number="localInterval" min="100" />
      </div>
      <div class="button-group">
        <button @click="saveSettings">Save</button>
        <button @click="close">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { ref, watch, defineComponent } from 'vue';
import { Store, load } from '@tauri-apps/plugin-store';
import GenericPopup from './GenericPopup.vue';

export default defineComponent({
  props: {
    visible: { type: Boolean, default: false },
    interval: { type: Number, required: true }
  },
  components: {
    GenericPopup
  },
  emits: ['update:visible', 'update:interval'],
  setup(props, { emit }) {
    const localInterval = ref(props.interval);
    let store: Store;

    const showError = ref(false);

    watch(() => props.visible, (val) => {
      if (val) localInterval.value = props.interval;
    });

    const saveSettings = async () => {
      if (!localInterval.value) return;

      if (localInterval.value < 500) {
        localInterval.value = 500;
        showError.value = true;
        return;
      }

      emit('update:interval', localInterval.value);
      emit('update:visible', false);

      if (!store) {
        store = await load('store.json', { autoSave: false, defaults: { interval: props.interval } });
      }
      await store.set('interval', { value: localInterval.value });
      await store.save();
    };

    const close = () => emit('update:visible', false);

    const overlayClick = (event: MouseEvent) => {
      if (event.target === event.currentTarget) close();
    };

    return { localInterval, saveSettings, close, overlayClick, showError };
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
