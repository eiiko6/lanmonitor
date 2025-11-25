<template>
  <div class="monitor">
    <section class="block">
      <h2>System</h2>

      <div v-if="system">
        <p><strong>CPU:</strong> {{ system.cpu_usage.toFixed(1) }}%</p>
        <p>
          <strong>RAM:</strong>
          {{ system.memory_used }} / {{ system.memory_total }} MB
        </p>
      </div>
      <p v-else class="loading">Loading system info…</p>
    </section>

    <section class="block">
      <h2>Temperatures</h2>

      <ul v-if="temps.length > 0">
        <li v-for="t in temps" :key="t.label">
          {{ t.label }} — {{ Number(t.temp).toFixed(1) }} °C
        </li>
      </ul>

      <p v-else class="loading">Loading temperatures…</p>
    </section>
  </div>
</template>

<script lang="ts">
import { ref, onMounted } from "vue";

export default {
  name: "Monitor",

  setup() {
    const system = ref<any>(null);
    const temps = ref<any[]>([]);

    async function refresh() {
      try {
        const sysRes = await fetch("http://localhost:8080/system");
        const tempRes = await fetch("http://localhost:8080/temps");

        system.value = await sysRes.json();
        temps.value = await tempRes.json();
        console.log(temps.value)
      } catch (e) {
        console.error("Monitoring fetch failed:", e);
      }
    }

    onMounted(() => {
      refresh();
      setInterval(refresh, 1000);
    });

    return { system, temps };
  },
};
</script>

<style scoped>
.monitor {
  max-width: 400px;
  margin: 0 auto;
  font-family: sans-serif;
  padding: 1rem;
}

.block {
  margin-bottom: 1.5rem;
  padding: 1rem;
  border-radius: 8px;
  background: #f0f0f0;
}

h2 {
  margin-top: 0;
}

.loading {
  opacity: 0.6;
  font-style: italic;
}
</style>
