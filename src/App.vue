<script setup lang="ts">
import { ref, onMounted } from "vue";
import { listen } from '@tauri-apps/api/event';

type ClipboardEvent = {
  content: string;
};

// 时间戳弹窗相关状态
const showTimestampPopup = ref(false);
const timestampInfo = ref<{
  timestamp: string;
  utcTime: string;
  localTime: string;
} | null>(null);

// 检测是否为有效时间戳
function isValidTimestamp(content: string): boolean {
  // 去除空白字符
  const trimmed = content.trim();
  
  // 检查是否为纯数字
  if (!/^\d+$/.test(trimmed)) {
    return false;
  }
  
  const length = trimmed.length;
  let timestamp: number;
  
  // 根据位数确定时间戳类型
  if (length === 10) {
    // 秒级时间戳
    timestamp = parseInt(trimmed) * 1000;
  } else if (length === 13) {
    // 毫秒级时间戳
    timestamp = parseInt(trimmed);
  } else if (length === 16) {
    // 微秒级时间戳，转换为毫秒
    timestamp = Math.floor(parseInt(trimmed) / 1000);
  } else {
    return false;
  }
  
  // 检查时间戳是否 > 0
  if (timestamp <= 0) {
    return false;
  }
  
  // 检查是否 < 9999年 (9999-12-31 23:59:59 UTC)
  const maxTimestamp = new Date('9999-12-31T23:59:59Z').getTime();
  if (timestamp >= maxTimestamp) {
    return false;
  }
  
  return true;
}

// 格式化时间戳
function formatTimestamp(content: string): { timestamp: string; utcTime: string; localTime: string } | null {
  const trimmed = content.trim();
  const length = trimmed.length;
  let timestamp: number;
  let hasMs = false;
  let hasUs = false;
  
  if (length === 10) {
    timestamp = parseInt(trimmed) * 1000;
  } else if (length === 13) {
    timestamp = parseInt(trimmed);
    hasMs = true;
  } else if (length === 16) {
    const usTimestamp = parseInt(trimmed);
    timestamp = Math.floor(usTimestamp / 1000);
    hasMs = true;
    hasUs = true;
  } else {
    return null;
  }
  
  const date = new Date(timestamp);
  
  // 格式化UTC时间
  const utcYear = date.getUTCFullYear();
  const utcMonth = String(date.getUTCMonth() + 1).padStart(2, '0');
  const utcDay = String(date.getUTCDate()).padStart(2, '0');
  const utcHours = String(date.getUTCHours()).padStart(2, '0');
  const utcMinutes = String(date.getUTCMinutes()).padStart(2, '0');
  const utcSeconds = String(date.getUTCSeconds()).padStart(2, '0');
  
  let utcTime = `${utcYear}-${utcMonth}-${utcDay} ${utcHours}:${utcMinutes}:${utcSeconds}`;
  
  if (hasMs) {
    const ms = String(date.getUTCMilliseconds()).padStart(3, '0');
    utcTime += `.${ms}`;
    
    if (hasUs) {
      const us = String(parseInt(trimmed) % 1000).padStart(3, '0');
      utcTime += `.${us}`;
    }
  }
  
  // 格式化本地时间
  const localYear = date.getFullYear();
  const localMonth = String(date.getMonth() + 1).padStart(2, '0');
  const localDay = String(date.getDate()).padStart(2, '0');
  const localHours = String(date.getHours()).padStart(2, '0');
  const localMinutes = String(date.getMinutes()).padStart(2, '0');
  const localSeconds = String(date.getSeconds()).padStart(2, '0');
  
  let localTime = `${localYear}-${localMonth}-${localDay} ${localHours}:${localMinutes}:${localSeconds}`;
  
  if (hasMs) {
    const ms = String(date.getMilliseconds()).padStart(3, '0');
    localTime += `.${ms}`;
    
    if (hasUs) {
      const us = String(parseInt(trimmed) % 1000).padStart(3, '0');
      localTime += `.${us}`;
    }
  }
  
  return { timestamp: trimmed, utcTime, localTime };
}

// 格式化当前时间
function formatCurrentTime(): { timestamp: string; utcTime: string; localTime: string } {
  const now = new Date();
  const timestamp = String(now.getTime());
  
  // 格式化UTC时间
  const utcYear = now.getUTCFullYear();
  const utcMonth = String(now.getUTCMonth() + 1).padStart(2, '0');
  const utcDay = String(now.getUTCDate()).padStart(2, '0');
  const utcHours = String(now.getUTCHours()).padStart(2, '0');
  const utcMinutes = String(now.getUTCMinutes()).padStart(2, '0');
  const utcSeconds = String(now.getUTCSeconds()).padStart(2, '0');
  const ms = String(now.getUTCMilliseconds()).padStart(3, '0');
  
  const utcTime = `${utcYear}-${utcMonth}-${utcDay} ${utcHours}:${utcMinutes}:${utcSeconds}.${ms}`;
  
  // 格式化本地时间
  const localYear = now.getFullYear();
  const localMonth = String(now.getMonth() + 1).padStart(2, '0');
  const localDay = String(now.getDate()).padStart(2, '0');
  const localHours = String(now.getHours()).padStart(2, '0');
  const localMinutes = String(now.getMinutes()).padStart(2, '0');
  const localSeconds = String(now.getSeconds()).padStart(2, '0');
  const localMs = String(now.getMilliseconds()).padStart(3, '0');
  
  const localTime = `${localYear}-${localMonth}-${localDay} ${localHours}:${localMinutes}:${localSeconds}.${localMs}`;
  
  return { timestamp, utcTime, localTime };
}

// 显示时间戳弹窗
function showPopup(timestamp: string, utcTime: string, localTime: string) {
  timestampInfo.value = { timestamp, utcTime, localTime };
  showTimestampPopup.value = true;
}

// 初始化：显示当前时间
onMounted(() => {
  const currentTime = formatCurrentTime();
  timestampInfo.value = { timestamp: currentTime.timestamp, utcTime: currentTime.utcTime, localTime: currentTime.localTime };
  showTimestampPopup.value = true;
});

listen<ClipboardEvent>('clipboard', (event) => {
  const content = event.payload.content;
  // 检测是否为时间戳
  if (isValidTimestamp(content)) {
    const formatted = formatTimestamp(content);
    if (formatted) {
      showPopup(formatted.timestamp, formatted.utcTime, formatted.localTime);
    }
  }
});

</script>

<template>
  <!-- 时间戳内容 -->
  <div v-if="showTimestampPopup && timestampInfo" class="timestamp-content">
    <div class="time-row">
      <span class="time-label">时间戳:</span>
      <span class="time-value">{{ timestampInfo.timestamp }}</span>
    </div>
    <div class="time-row">
      <span class="time-label">UTC:</span>
      <span class="time-value">{{ timestampInfo.utcTime }}</span>
    </div>
    <div class="time-row">
      <span class="time-label">本地:</span>
      <span class="time-value">{{ timestampInfo.localTime }}</span>
    </div>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 12px;
}

/* 时间戳内容样式 */
.timestamp-content {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.time-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  min-height: 0;
}

.time-label {
  font-weight: 500;
  font-size: 11px;
  color: #666;
  min-width: 36px;
  flex-shrink: 0;
  padding-top: 2px;
}

.time-value {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 11px;
  color: #0f0f0f;
  word-break: break-all;
  line-height: 1.5;
  flex: 1;
  overflow-wrap: break-word;
}

@media (prefers-color-scheme: dark) {
  .time-label {
    color: #999;
  }
  
  .time-value {
    color: #f6f6f6;
  }
}

</style>