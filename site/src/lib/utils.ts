import { toast } from "@zerodevx/svelte-toast";

export const debounce = (f, timeout = 250) => {
  let timer;
  return (...v) => {
    clearTimeout(timer);
    timer = setTimeout(() => f(...v), timeout);
  };
};

export const toastSuccess = (msg: string) =>
  toast.push(msg, {
    theme: {
      "--toastBackground": "#48BB78",
      "--toastProgressBackground": "#2F855A",
    },
  });

export const toastError = (msg: string) =>
  toast.push(msg, {
    theme: {
      "--toastBackground": "#F56565",
      "--toastProgressBackground": "#C53030",
    },
  });

export const measureTime = (f) => {
  const t0 = performance.now();
  f();
  const t1 = performance.now();
  return t1 - t0;
};

export const copyTextToClipboard = (text: string) =>
  navigator.clipboard
    .writeText(text)
    .then(() => toastSuccess("Copied to clipboard"))
    .catch((e) => {
      console.error("clipboard.writeText failed", e);
      toastError("Could not access the clipboard");
    });
