import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

// see https://v2.tauri.app/plugin/logging/#logging
function forwardConsole(
  fnName: "log" | "debug" | "info" | "warn" | "error" | "trace",
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message.toString());
  };
}

let logForwardingInitialized = false;
export function initLogForwarding() {
  if (logForwardingInitialized) {
    const backtrace = new Error().stack;
    warn(`attempted to initialize log forwarding twice\nBacktrace:\n${backtrace}`);
    return;
  }

  forwardConsole("trace", trace);
  forwardConsole("log", info);
  forwardConsole("debug", debug);
  forwardConsole("info", info);
  forwardConsole("warn", warn);
  forwardConsole("error", error);
  logForwardingInitialized = true;
}
