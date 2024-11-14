import { Subject } from '~/util/observer';

export * from './ui';

export const moduleName = '{{ rootInfo.bJson.name }}';
export const componentName = 'main';

export const subject = new Subject();
