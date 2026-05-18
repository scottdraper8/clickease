export type Key =
  | { Char: string }
  | "Control"
  | "Shift"
  | "Alt"
  | "Meta"
  | "Escape"
  | "Enter"
  | "Backspace"
  | "Tab"
  | "Space"
  | "Up"
  | "Down"
  | "Left"
  | "Right"
  | "F1"
  | "F2"
  | "F3"
  | "F4"
  | "F5"
  | "F6"
  | "F7"
  | "F8"
  | "F9"
  | "F10"
  | "F11"
  | "F12";

export type MouseButton = "Left" | "Right" | "Middle";

export type Command =
  | { KeyPress: Key }
  | { KeyDown: Key }
  | { KeyUp: Key }
  | { MouseClick: MouseButton }
  | { MouseDown: MouseButton }
  | { MouseUp: MouseButton }
  | { MouseMove: { x: number; y: number } }
  | { Wait: { secs: number; nanos: number } };

export interface Schedule {
  id?: string;
  name: string;
  sequence: Command[];
  interval: { secs: number; nanos: number };
  active_duration: { secs: number; nanos: number };
  repeat_after?: { secs: number; nanos: number };
}

export type ScheduleStatus = "Idle" | "Running" | "Paused" | "Completed";
