import {
  ICaptureService,
  IDiffService,
  ILoadInteractionResponse,
} from '../../services/diff';
import { DiffRfcBaseState } from '@useoptic/cli-shared/build/diffs/diff-rfc-base-state';

export interface DiffSessionConfig {
  loadInteraction: (pointer: string) => Promise<ILoadInteractionResponse>;
  rfcBaseState: DiffRfcBaseState;
}
export interface InteractiveDiffSessionConfig {
  loadInteraction: (pointer: string) => Promise<ILoadInteractionResponse>;
  captureService: ICaptureService;
  diffService: IDiffService;
  rfcBaseState: DiffRfcBaseState;
}
