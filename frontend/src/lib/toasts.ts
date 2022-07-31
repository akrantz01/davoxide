import { Toaster as BaseToaster, Intent } from '@blueprintjs/core';

const Toaster = BaseToaster.create({ canEscapeKeyClear: true });

export const toast = (message: string, intent: Intent = Intent.NONE) =>
  Toaster.show({ message, intent, timeout: 2500 });

export const danger = (message: string) => toast(message, Intent.DANGER);
export const success = (message: string) => toast(message, Intent.SUCCESS);
export const warning = (message: string) => toast(message, Intent.WARNING);

export default Toaster;
