import { getContext, setContext } from "svelte";

export function setService<T>(key: string | Symbol, service: T): T {
  setContext(key, service);
  return service;
}

export function getService<T>(key: string | Symbol): T {
  return getContext(key);
}

export function defineService<T>(key: string | Symbol = Symbol()): [() => T, (service: T) => T] {
  return [() => getService<T>(key), (service: T) => setService(key, service)];
}
