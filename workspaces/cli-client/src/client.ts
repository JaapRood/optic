import { IOpticTaskRunnerConfig, IUser } from '@useoptic/cli-config';
import { JsonHttpClient } from '@useoptic/client-utilities';
import { TrackingEventBase } from '@useoptic/analytics/lib/interfaces/TrackingEventBase';

class Client {
  constructor(private baseUrl: string) {}

  getIdentity() {
    const url = `${this.baseUrl}/identity`;
    return JsonHttpClient.getJsonWithoutHandlingResponse(url);
  }

  setIdentity(user: IUser | null) {
    if (user) {
      const url = `${this.baseUrl}/identity`;
      return JsonHttpClient.putJson(url, { user });
    }
  }

  postTrackingEvents(apiName: string, events: TrackingEventBase<any>[]) {
    const url = `${this.baseUrl}/tracking/events`;
    return JsonHttpClient.postJson(url, { apiName, events });
  }

  findSession(
    path: string,
    taskConfig: IOpticTaskRunnerConfig | null,
    captureId: string | null
  ): Promise<{ session: { id: string } }> {
    const url = `${this.baseUrl}/sessions`;
    return JsonHttpClient.postJson(url, { path, taskConfig, captureId });
  }

  markCaptureAsCompleted(specId: string, captureId: string) {
    const url = `${this.baseUrl}/specs/${specId}/captures/${captureId}/status`;
    return JsonHttpClient.putJson(url, { status: 'completed' });
  }

  stopDaemon() {
    const url = `${this.baseUrl}/commands`;
    return JsonHttpClient.postJson(url, { type: 'shutdown' });
  }
}

export { Client };
