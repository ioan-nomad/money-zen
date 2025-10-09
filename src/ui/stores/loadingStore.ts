import { writable } from 'svelte/store';

interface LoadingState {
  isLoading: boolean;
  loadingText?: string;
  operations: Set<string>;
}

function createLoadingStore() {
  const { subscribe, update } = writable<LoadingState>({
    isLoading: false,
    loadingText: undefined,
    operations: new Set()
  });

  return {
    subscribe,

    start(operation: string, text?: string) {
      update(state => {
        state.operations.add(operation);
        return {
          isLoading: true,
          loadingText: text || state.loadingText,
          operations: state.operations
        };
      });
    },

    stop(operation: string) {
      update(state => {
        state.operations.delete(operation);
        const isLoading = state.operations.size > 0;
        return {
          isLoading,
          loadingText: isLoading ? state.loadingText : undefined,
          operations: state.operations
        };
      });
    },

    reset() {
      update(() => ({
        isLoading: false,
        loadingText: undefined,
        operations: new Set()
      }));
    }
  };
}

export const loadingStore = createLoadingStore();