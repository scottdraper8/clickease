import { invoke } from "@tauri-apps/api/core";
import { Schedule } from "../types/simulator";

export class ApiClient {
  static async startSchedule(schedule: Schedule): Promise<void> {
    try {
      await invoke("start_schedule", { schedule });
    } catch (error) {
      console.error("Failed to start schedule:", error);
      throw error;
    }
  }

  static async stopSchedule(id: string): Promise<boolean> {
    try {
      return await invoke("stop_schedule", { id });
    } catch (error) {
      console.error("Failed to stop schedule:", error);
      throw error;
    }
  }

  static async stopAllSchedules(): Promise<void> {
    try {
      await invoke("stop_all_schedules");
    } catch (error) {
      console.error("Failed to stop all schedules:", error);
      throw error;
    }
  }

  static async getPermissions(): Promise<{
    has_accessibility: boolean;
    is_admin: boolean;
  }> {
    try {
      return await invoke("get_permissions");
    } catch (error) {
      console.error("Failed to get permissions:", error);
      throw error;
    }
  }

  static async requestPermissions(): Promise<void> {
    try {
      await invoke("request_permissions");
    } catch (error) {
      console.error("Failed to request permissions:", error);
      throw error;
    }
  }

  static async greet(name: string): Promise<string> {
    return await invoke("greet", { name });
  }
}
