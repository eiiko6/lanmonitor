<template>
  <div id="page">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" :key="$route.path" />
      </transition>
    </router-view>
  </div>
</template>

<script>
import { onMounted, onUnmounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';

export default {
};
</script>

<style scoped>
#page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  min-height: 100vh;
  /* min-width: 100vw; */
  margin-top: 20px;
  text-align: center;
  background-size: cover;
  background-repeat: no-repeat;
  background-attachment: fixed;
  z-index: 1;
}

.navbar {
  z-index: 2;
}

#footer {
  position: fixed;
  bottom: 0;
  text-align: center;
  padding: 8px;
  font-size: .86em;
  opacity: .82;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@media (max-width: 700px) {
  #page {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background-attachment: scroll;
    transform: none;
    overflow-x: hidden;
    width: 100%;
  }
}
</style>
