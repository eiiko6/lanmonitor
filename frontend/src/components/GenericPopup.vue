<template>
  <div v-if="visible" class="modal-overlay" @click="overlayClick">
    <div class="modal">
      <h2>{{ title }}</h2>
      <p>{{ message }}</p>
      <div class="button-group">
        <button @click="close">Close</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    visible: { type: Boolean, default: false },
    title: { type: String, required: true },
    message: { type: String, required: true },
  },
  emits: ['update:visible'],
  setup(_props, { emit }) {
    const close = () => emit('update:visible', false);

    const overlayClick = (event: MouseEvent) => {
      if (event.target === event.currentTarget) close();
    };

    return { close, overlayClick };
  },
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
  z-index: 200;
}

.modal {
  background: #43518E33;
  padding: 2rem;
  border-radius: 15px;
  min-width: 300px;
  text-align: center;
  backdrop-filter: blur(50px) brightness(85%) saturate(120%);
  border: 1px solid #dddddd44;
  box-shadow: inset 0 0 25px #dddddd20, 0 4px 30px rgba(0, 0, 0, 0.1);
}

.button-group {
  margin-top: 1rem;
  display: flex;
  justify-content: center;
}
</style>
