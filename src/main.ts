import "./styles/global.css";
import { ThemeManager } from "./services/theme";
import { ApiClient } from "./services/api";
import { Schedule, Command, Key } from "./types/simulator";
import { listen } from "@tauri-apps/api/event";
import { type } from "@tauri-apps/plugin-os";

ThemeManager.init();

const themeToggle = document.getElementById("theme-toggle");
const stopAllBtn = document.getElementById("stop-all-btn");
const scheduleList = document.getElementById("schedule-list");
const permissionBanner = document.getElementById("permission-banner");
const requestPermissionBtn = document.getElementById("request-permission-btn");
const selectedKeysDisplay = document.getElementById("selected-keys-display");
const startBtn = document.getElementById("start-btn");
const specialKeysToggle = document.getElementById("special-keys-toggle");

const modePress = document.getElementById("mode-press");
const modeHold = document.getElementById("mode-hold");
const holdSettings = document.getElementById("hold-settings");

let selectedKeys: string[] = [];
let currentMode: "press" | "hold" = "press";
let shiftMode = false;
const activeSchedules: Map<
  string,
  { schedule: Schedule; element: HTMLElement; isPaused: boolean }
> = new Map();

const shiftMappings: Record<string, string> = {
  "1": "!",
  "2": "@",
  "3": "#",
  "4": "$",
  "5": "%",
  "6": "^",
  "7": "&",
  "8": "*",
  "9": "(",
  "0": ")",
  "-": "_",
  "=": "+",
  "[": "{",
  "]": "}",
  "\\": "|",
  ";": ":",
  "'": '"',
  ",": "<",
  ".": ">",
  "/": "?",
};

function updateThemeButtonText() {
  const isDark = document.documentElement.classList.contains("dark");
  if (themeToggle) {
    themeToggle.innerHTML = isDark
      ? '<span class="uppercase tracking-widest text-xs font-bold">Light Mode</span>'
      : '<span class="uppercase tracking-widest text-xs font-bold">Dark Mode</span>';
  }
}

window.onerror = (msg, url, line, col, error) => {
  console.error("Window Error:", msg, "at", url, ":", line, ":", col, error);
  return false;
};

window.addEventListener("unhandledrejection", (event) => {
  console.error("Unhandled Promise Rejection:", event.reason);
});

updateThemeButtonText();

async function checkPermissions() {
  try {
    const status = await ApiClient.getPermissions();
    let platform = "unknown";
    try {
      platform = await type();
    } catch (e) {
      console.warn(
        "tauri-plugin-os type() failed, falling back to userAgent",
        e,
      );
      platform = navigator.userAgent.toLowerCase().includes("mac")
        ? "macos"
        : "other";
    }

    if (!status.has_accessibility && platform === "macos") {
      permissionBanner?.classList.remove("hidden");
    } else {
      permissionBanner?.classList.add("hidden");
    }
  } catch (err) {
    console.error("Permission check failed:", err);
  }
}
checkPermissions();

themeToggle?.addEventListener("click", () => {
  ThemeManager.toggle();
  updateThemeButtonText();
});

specialKeysToggle?.addEventListener("click", () => {
  shiftMode = !shiftMode;
  specialKeysToggle.classList.toggle("active", shiftMode);

  // Trigger pulse animation only on affected keys
  document.querySelectorAll(".key").forEach((k) => {
    const btn = k as HTMLElement;
    const key = btn.dataset.key;
    if (key && shiftMappings[key]) {
      k.classList.remove("key-highlight");
      void (k as HTMLElement).offsetWidth; // Trigger reflow
      k.classList.add("key-highlight");
    }
  });

  selectedKeys = selectedKeys.map((k) => {
    if (shiftMode) {
      return shiftMappings[k] || k;
    } else {
      const unshifted = Object.keys(shiftMappings).find(
        (key) => shiftMappings[key] === k,
      );
      return unshifted || k;
    }
  });

  updateKeyboardLabels();
  updateSelectedKeysDisplay();
});

function updateKeyboardLabels() {
  document.querySelectorAll(".key").forEach((el) => {
    const btn = el as HTMLElement;
    const key = btn.dataset.key;
    if (key && shiftMappings[key]) {
      btn.textContent = shiftMode ? shiftMappings[key] : key;
    }
  });
}

document.querySelectorAll(".key").forEach((el) => {
  const keyBtn = el as HTMLElement;
  keyBtn.addEventListener("click", () => {
    const keyValue = keyBtn.dataset.key;
    if (!keyValue) return;

    const displayValue = keyBtn.textContent?.trim() || keyValue;

    if (selectedKeys.includes(displayValue)) {
      selectedKeys = selectedKeys.filter((k) => k !== displayValue);
      keyBtn.classList.remove("active");
    } else {
      if (selectedKeys.length < 3) {
        selectedKeys.push(displayValue);
        keyBtn.classList.add("active");
      }
    }
    updateSelectedKeysDisplay();
  });
});

function updateSelectedKeysDisplay() {
  if (selectedKeysDisplay) {
    selectedKeysDisplay.textContent =
      selectedKeys.length > 0 ? selectedKeys.join(" + ") : "No keys selected";
  }
}

document.querySelectorAll(".neu-step-btn").forEach((el) => {
  const btn = el as HTMLElement;
  btn.addEventListener("click", () => {
    const inputId = btn.dataset.input;
    const step = parseFloat(btn.dataset.step || "1");
    if (inputId) {
      const input = document.getElementById(inputId) as HTMLInputElement;
      const newVal = (parseFloat(input.value || "0") + step).toFixed(1);
      input.value = newVal;
    }
  });
});

modePress?.addEventListener("click", () => {
  currentMode = "press";
  modePress.classList.add("active");
  modeHold?.classList.remove("active");
  holdSettings?.classList.remove("show");
});

modeHold?.addEventListener("click", () => {
  currentMode = "hold";
  modeHold.classList.add("active");
  modePress?.classList.remove("active");
  holdSettings?.classList.add("show");
});

startBtn?.addEventListener("click", async () => {
  if (selectedKeys.length === 0) {
    alert("Please select at least one key.");
    return;
  }

  const intervalVal = parseFloat(
    (document.getElementById("interval") as HTMLInputElement).value || "1.0",
  );
  const durationVal = parseFloat(
    (document.getElementById("duration") as HTMLInputElement).value || "60",
  );
  const holdDurationVal = parseFloat(
    (document.getElementById("hold-duration") as HTMLInputElement).value || "5",
  );

  const sequence: Command[] = [];

  if (currentMode === "press") {
    selectedKeys.forEach((k) => {
      sequence.push({ KeyPress: mapStringToKey(k) });
    });
  } else {
    selectedKeys.forEach((k) => {
      sequence.push({ KeyDown: mapStringToKey(k) });
    });
    sequence.push({
      Wait: {
        secs: Math.floor(holdDurationVal),
        nanos: Math.round((holdDurationVal % 1) * 1e9),
      },
    });
    selectedKeys.forEach((k) => {
      sequence.push({ KeyUp: mapStringToKey(k) });
    });
  }

  const scheduleId = crypto.randomUUID();
  const schedule: Schedule = {
    id: scheduleId,
    name: `${currentMode.toUpperCase()}: ${selectedKeys.join("+")}`,
    sequence,
    interval: {
      secs: Math.floor(intervalVal),
      nanos: Math.round((intervalVal % 1) * 1e9),
    },
    active_duration: { secs: durationVal * 60, nanos: 0 },
  };

  try {
    await ApiClient.startSchedule(schedule);
    addScheduleToList(schedule);
    resetDashboard();
  } catch (err) {
    console.error("Failed to start schedule:", err);
  }
});

function resetDashboard() {
  selectedKeys = [];
  shiftMode = false;
  specialKeysToggle?.classList.remove("active");
  document
    .querySelectorAll(".key")
    .forEach((k) => k.classList.remove("active"));
  updateKeyboardLabels();
  updateSelectedKeysDisplay();
}

function mapStringToKey(s: string): Key {
  if (s.length === 1) return { Char: s.toLowerCase() };
  return s as Key;
}

listen("schedule-completed", (event) => {
  const id = event.payload as string;
  removeScheduleFromList(id);
});

listen("schedules-stopped", () => {
  activeSchedules.forEach((_, id) => removeScheduleFromList(id));
});

listen("schedule-tick", (event) => {
  const payload = event.payload as {
    id: string;
    remaining_secs: number;
    is_paused: boolean;
  };
  const item = activeSchedules.get(payload.id);
  if (item) {
    item.isPaused = payload.is_paused;
    const timeEl = item.element.querySelector(".time-remaining");
    if (timeEl) {
      const minutes = Math.floor(payload.remaining_secs / 60);
      const seconds = payload.remaining_secs % 60;
      timeEl.textContent = `${minutes}:${seconds.toString().padStart(2, "0")}`;
    }

    const pauseBtn = item.element.querySelector(".pause-btn");
    if (pauseBtn) {
      pauseBtn.textContent = item.isPaused ? "RESUME" : "PAUSE";
      pauseBtn.classList.toggle("text-amber-500", !item.isPaused);
      pauseBtn.classList.toggle("text-green-500", item.isPaused);
    }
  }
});

function updateStopAllVisibility() {
  if (activeSchedules.size > 0) {
    stopAllBtn?.classList.remove("hidden");
  } else {
    stopAllBtn?.classList.add("hidden");
  }
}

function addScheduleToList(schedule: Schedule) {
  if (!scheduleList || !schedule.id) return;

  if (activeSchedules.size === 0) {
    scheduleList.innerHTML = "";
  }

  const card = document.createElement("div");
  card.className = "neu-card flex flex-col gap-4 animate-in";
  card.innerHTML = `
    <div class="flex justify-between items-start">
      <div class="flex flex-col gap-1">
        <h3 class="font-bold text-sm text-[var(--primary)] uppercase tracking-wider">${
          schedule.name
        }</h3>
        <p class="text-[10px] opacity-50 font-mono">INT: ${
          schedule.interval.secs + schedule.interval.nanos / 1e9
        }s | REM: <span class="time-remaining">--:--</span></p>
      </div>
      <div class="flex gap-2">
        <button class="neu-button pause-btn py-2 px-3 text-[10px] font-bold text-amber-500">
          PAUSE
        </button>
        <button class="neu-button stop-btn py-2 px-3 text-[10px] font-bold text-red-500">
          STOP
        </button>
      </div>
    </div>
  `;

  card.querySelector(".pause-btn")?.addEventListener("click", async () => {
    const item = activeSchedules.get(schedule.id!);
    if (item && schedule.id) {
      if (!item.isPaused) {
        await ApiClient.pauseSchedule(schedule.id);
        item.isPaused = true;
      } else {
        await ApiClient.resumeSchedule(schedule.id);
        item.isPaused = false;
      }
      const pauseBtn = item.element.querySelector(".pause-btn")!;
      pauseBtn.textContent = item.isPaused ? "RESUME" : "PAUSE";
      pauseBtn.classList.toggle("text-amber-500", !item.isPaused);
      pauseBtn.classList.toggle("text-green-500", item.isPaused);
    }
  });

  card.querySelector(".stop-btn")?.addEventListener("click", async () => {
    if (schedule.id) {
      await ApiClient.stopSchedule(schedule.id);
      removeScheduleFromList(schedule.id);
    }
  });

  activeSchedules.set(schedule.id, { schedule, element: card, isPaused: false });
  scheduleList.appendChild(card);
  updateStopAllVisibility();
}

function removeScheduleFromList(id: string) {
  const item = activeSchedules.get(id);
  if (item) {
    item.element.classList.add(
      "opacity-0",
      "scale-95",
      "transition-all",
      "duration-300",
    );
    setTimeout(() => {
      item.element.remove();
      activeSchedules.delete(id);
      if (activeSchedules.size === 0) {
        if (scheduleList) {
          scheduleList.innerHTML = `<div class="opacity-40 italic text-center text-sm py-12">No active jobs</div>`;
        }
      }
      updateStopAllVisibility();
    }, 300);
  }
}

stopAllBtn?.addEventListener("click", async () => {
  await ApiClient.stopAllSchedules();
});

requestPermissionBtn?.addEventListener("click", async () => {
  await ApiClient.requestPermissions();
  setTimeout(checkPermissions, 5000);
});

console.log("Clickease Dashboard Initialized.");
