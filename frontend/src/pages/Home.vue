<template>
  <div class="monitor">
    <div class="column">
      <section class="block">
        <h2>Static System</h2>

        <div v-if="staticSystem">
          <p><strong>OS:</strong> {{ staticSystem.os_version }}</p>
          <p><strong>Kernel:</strong> {{ staticSystem.kernel_version }}</p>
          <p><strong>Distribution:</strong> {{ staticSystem.distribution_id }}</p>
          <p><strong>Age:</strong> {{ staticSystem.os_age }} days (root partition filesystem)</p>
          <p><strong>CPU Cores:</strong> {{ staticSystem.core_count }}</p>
        </div>
        <p v-else class="loading">Loading static info…</p>
      </section>

      <section class="block">
        <h2>System</h2>

        <div v-if="system">
          <p><strong>Uptime:</strong> {{ formatUptime(system.uptime) }}</p>

          <p><strong>CPU usage:</strong> {{ system.cpu_usage.toFixed(1) }}%</p>

          <p>
            <strong>RAM usage:</strong>
            {{ (system.used_memory / 1000000000).toPrecision(4) }} /
            {{ (system.total_memory / 1000000000).toPrecision(4) }} GB -
            {{ (100 * (system.used_memory / 1000000000) / (system.total_memory / 1000000000)).toPrecision(4)
            }}%
          </p>

          <p><strong>GPU usage:</strong> {{ system.gpu_usage }}%</p>

          <p><strong>GPU power:</strong> {{ system.gpu_power_usage }}W / {{ system.gpu_power_cap }}W</p>
        </div>
        <p v-else class="loading">Loading system info…</p>
      </section>
    </div>

    <section class="block">
      <h2>Temperatures</h2>

      <table v-if="temps.length > 0">
        <thead>
          <tr>
            <th>ID</th>
            <th>Label</th>
            <th>Temp (°C)</th>
            <th>Max (°C)</th>
            <th>Critical (°C)</th>
          </tr>
        </thead>

        <tbody>
          <tr v-for="t in temps" :key="t.label">
            <td>{{ t.id }}</td>
            <td>{{ t.label }}</td>
            <td>{{ Number(t.temp).toFixed(1) }}</td>
            <td>{{ t.max }}</td>
            <td>{{ t.critical === "unknown" ? "" : t.critical }}</td>
          </tr>
        </tbody>
      </table>

      <p v-else class="loading">Loading components…</p>
    </section>
  </div>
</template>

<script lang="ts">
import { ref, onMounted } from "vue";

export default {
  name: "Monitor",

  setup() {
    const daemonIP = "http://192.168.1.183:8081";

    const staticSystem = ref<any>(null);
    const system = ref<any>(null);
    const temps = ref<any[]>([]);

    async function loadStatic() {
      try {
        const res = await fetch(daemonIP + "/static-system");
        staticSystem.value = await res.json();
      } catch (e) {
        console.error("Static system fetch failed:", e);
      }
    }
    async function refresh() {
      try {
        const sysRes = await fetch(daemonIP + "/system");
        const tempRes = await fetch(daemonIP + "/temperatures");

        system.value = await sysRes.json();
        temps.value = (await tempRes.json()).sort((a: any, b: any) =>
          a.id.localeCompare(b.id)
        );
        console.log(temps.value)
      } catch (e) {
        console.error("Monitoring fetch failed:", e);
      }
    }

    function formatUptime(seconds: number): string {
      const h = Math.floor(seconds / 3600);
      const m = Math.floor((seconds % 3600) / 60);
      const s = seconds % 60;

      const hh = h > 0 ? `${h}h ` : '';
      const mm = `${m}min `;
      const ss = s > 0 ? `${s}s ` : '';

      return `${hh}${mm}${ss}`;
    }

    onMounted(() => {
      loadStatic();
      refresh();
      setInterval(refresh, 1000);
    });

    return { staticSystem, system, temps, formatUptime };
  },
};
</script>

<style scoped>
.monitor {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  justify-content: center;
  /* align-items: center; */
  gap: 2rem;
  margin: 0 auto;
  font-family: sans-serif;
  padding: 1rem;
  text-align: left;
}

.column {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}


.block {
  padding: 2rem;
  border-radius: 15px;
  background: #43518E33;
  backdrop-filter: blur(50px) brightness(85%) saturate(120%);
  border: 1px solid #dddddd44;
  box-shadow:
    inset 0 0 15px #dddddd35,
    0 4px 30px rgba(0, 0, 0, 0.1);
}

h2 {
  margin-top: 0;
}

.loading {
  opacity: 0.6;
  /* font-style: italic; */
}
</style>
