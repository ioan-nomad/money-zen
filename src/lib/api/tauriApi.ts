import { invoke } from '@tauri-apps/api/core';
import { loadingStore } from '../../ui/stores/loadingStore';
import { notificationStore } from '../../ui/stores/notificationStore';

export class TauriApi {
  private static async invokeWithLoading<T>(
    command: string,
    args?: Record<string, any>,
    options?: {
      loadingText?: string;
      showError?: boolean;
      showSuccess?: boolean;
      successMessage?: string;
    }
  ): Promise<T> {
    const operationId = `${command}-${Date.now()}`;

    try {
      loadingStore.start(operationId, options?.loadingText);

      const result = await invoke<T>(command, args);

      if (options?.showSuccess) {
        notificationStore.success(
          options.successMessage || 'Operațiune reușită!'
        );
      }

      return result;
    } catch (error) {
      console.error(`TauriApi error in ${command}:`, error);

      if (options?.showError !== false) {
        notificationStore.error(
          error instanceof Error ? error.message : 'Eroare necunoscută'
        );
      }

      throw error;
    } finally {
      loadingStore.stop(operationId);
    }
  }

  // Wrap all Tauri commands
  static get = <T>(command: string, args?: Record<string, any>) =>
    TauriApi.invokeWithLoading<T>(command, args, { showError: true });

  static create = <T>(command: string, args?: Record<string, any>) =>
    TauriApi.invokeWithLoading<T>(command, args, {
      showError: true,
      showSuccess: true,
      successMessage: 'Creat cu succes!'
    });

  static update = <T>(command: string, args?: Record<string, any>) =>
    TauriApi.invokeWithLoading<T>(command, args, {
      showError: true,
      showSuccess: true,
      successMessage: 'Actualizat cu succes!'
    });

  static delete = <T>(command: string, args?: Record<string, any>) =>
    TauriApi.invokeWithLoading<T>(command, args, {
      showError: true,
      showSuccess: true,
      successMessage: 'Șters cu succes!'
    });
}